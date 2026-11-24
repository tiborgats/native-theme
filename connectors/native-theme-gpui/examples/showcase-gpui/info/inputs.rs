//! What the Inputs page's widgets report about themselves (spec §3.4).

#[cfg(feature = "widgets")]
use gpui::{Hsla, Pixels};
use gpui_component::theme::Theme;
#[cfg(feature = "widgets")]
use native_theme::next::{FocusTrigger, PartStated, TextStyle};
use native_theme::next::{ResolvedTheme as NextTheme, Stroke, WidgetState};
use native_theme_gpui::next::{fill_of, padding_of, state_fill_of, text_color_of};
#[cfg(feature = "widgets")]
use native_theme_gpui::widgets::{
    CheckboxLook, SliderLook, SwitchLabelFont, SwitchLabelHome, SwitchLook, ThumbOutline,
};

#[cfg(feature = "widgets")]
use super::{LINE_AT_THE_MULTIPLIER, next_drawn_font, next_stated, percent_text};
use super::{
    WidgetInfo, chrome::input_background, claim, edge_claim, px_text, state_stroke_color_of,
    stated_claim, stroke_color_of, stroke_width_of,
};
use crate::demo::{InputField, NUMBER_STEP_ACTIVE, NUMBER_STEP_HOVER, WaveformKind};
use crate::support::SampleIcon;

/// A refined, enabled field's surface, which the showcase draws under the
/// field (`demo::text_field`), its line `width` wide: `r`'s
/// `input.field(NONE).fill` framed by that part's stroke, and under the
/// pointer by `input.field(HOVER).stroke.color` where a rule states it.
pub fn input_surface(info: WidgetInfo, r: &NextTheme, width: f32) -> WidgetInfo {
    let info = info
        .colors(edge_claim(
            "edge",
            "input.field(NONE).stroke.color",
            stroke_color_of(&r.input.field(WidgetState::NONE)),
            Some(width),
            "showcase",
        ))
        .colors(edge_claim(
            "hovered edge",
            "input.field(HOVER).stroke.color",
            state_stroke_color_of(&r.input.field(WidgetState::HOVER)),
            Some(width),
            "showcase",
        ));
    let unstroked = matches!(r.input.field(WidgetState::NONE).stroke, Stroke::None);
    surface(info, width, unstroked, "input")
}

/// A text area's surface, which the showcase draws under it
/// (`demo::textarea`, `demo::rows_textarea`), its line `width` wide:
/// [`input_surface`]'s, of the text area's own field,
/// `text_area.field(NONE)` and `text_area.field(HOVER)`.
pub fn text_area_surface(info: WidgetInfo, r: &NextTheme, width: f32) -> WidgetInfo {
    let info = info
        .colors(edge_claim(
            "edge",
            "text_area.field(NONE).stroke.color",
            stroke_color_of(&r.text_area.field(WidgetState::NONE)),
            Some(width),
            "showcase",
        ))
        .colors(edge_claim(
            "hovered edge",
            "text_area.field(HOVER).stroke.color",
            state_stroke_color_of(&r.text_area.field(WidgetState::HOVER)),
            Some(width),
            "showcase",
        ));
    let unstroked = matches!(r.text_area.field(WidgetState::NONE).stroke, Stroke::None);
    surface(info, width, unstroked, "text_area")
}

/// The surface line of a field of `widget` (`input` or `text_area`) over
/// its surface, stroked `width` wide, `unstroked` where its stroke is
/// stated as none. A 0 width draws no edge in either state, and the line
/// then names none.
fn surface(info: WidgetInfo, width: f32, unstroked: bool, widget: &str) -> WidgetInfo {
    let why = if unstroked {
        format!("{widget}.field(NONE).stroke is none")
    } else {
        format!("{widget}.field(NONE).stroke.width is 0")
    };
    let info = if width > 0.0 {
        info
    } else {
        info.config("edge", format!("none: {why}"))
    };
    // The frame is named only where one is stroked.
    let fill = if width > 0.0 {
        format!(
            "{widget}.field(NONE).fill framed by its stroke, and under the pointer by {widget}.field(HOVER).stroke.color, where a rule states it"
        )
    } else {
        format!("{widget}.field(NONE).fill, unframed in every state ({why})")
    };
    info.config("surface", format!("{fill}, drawn by the showcase under the field, which it leaves without a fill or an edge of its own: Input is Styled only and its root's one state is focused (input/input.rs, Input::render), where its own border in ring still shows over the surface"))
}

/// `info` of a disabled field over its surface (`demo::faded_surface`),
/// edged by `r`'s `input.field(NONE)` stroke and faded with it by `fade`,
/// the opacity `geometry::input_fill` gives the surface.
pub fn disabled_input_surface(info: WidgetInfo, r: &NextTheme, fade: Option<f32>) -> WidgetInfo {
    let edge = edge_claim(
        "edge, faded with the field by input.disabled_opacity",
        "input.field(NONE).stroke.color",
        stroke_color_of(&r.input.field(WidgetState::NONE)),
        stroke_width_of(&r.input.field(WidgetState::NONE)),
        "showcase",
    );
    let faded = edge.map(|c| match fade {
        Some(fade) => super::ColorClaim {
            value: c.value.opacity(fade),
            ..c
        },
        None => c,
    });
    info.colors(faded)
    .instance("disabled text", "input.text(DISABLED).color, which geometry::input_fill gives a disabled field's root in place of geometry::input's input.text(NONE).color, at half its alpha: the editor halves the alpha of a disabled field's text whatever its colour (gpui-base/input/base/element.rs, prepaint)")
        .config("surface", "geometry::input_fill's fill inside input.field(NONE)'s stroke, none where it is none or 0 wide, drawn by the showcase under the field, which it leaves without a fill or an edge of its own, the whole faded by input.disabled_opacity: gpui fades each quad on its own (gpui-pre/window.rs, paint_quad) and draws a quad's border over its own fill (gpui-pre-wgpu shaders.wgsl, fs_quad), so a fill under the line would show through the faded line")
}

/// What a field takes of the installed 0.8.0 theme, for its info.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldTaken {
    /// Its geometry builder applies: `geometry::input` on a refined field,
    /// `geometry::input_height` on a height-only one, `geometry::text_area`
    /// on a text area.
    pub builder: bool,
    /// It takes the theme's fill: `geometry::input_fill`'s on a refined
    /// field, its surface's on a text area.
    pub fill: bool,
    /// The showcase draws its surface under it, leaving it without a fill
    /// or an edge colour of its own (`demo::input_surface`,
    /// `demo::faded_surface`).
    pub surface: bool,
}

/// A single-line `Input` taking the refinement `field` names, with what it
/// takes of the installed 0.8.0 theme, `taken`: a `Refined` field
/// `geometry::input` and `geometry::input_fill`, over the surface the
/// showcase draws under it where both apply, a `HeightOnly` one
/// `geometry::input_height`. Its geometry lines are recorded where
/// `demo::text_field` applies the builders.
pub fn input(t: &Theme, field: InputField, taken: FieldTaken) -> WidgetInfo {
    let info = WidgetInfo::new("Input");
    let info = match field {
        InputField::Refined => info,
        InputField::HeightOnly => info.variant("control height only"),
    };
    // geometry::input_fill gives a refined field the platform's fill, in
    // place of upstream's (input/input.rs:800, then :808); its geometry line
    // names the fields. Where the showcase draws the field's surface, it
    // leaves the field without a fill or an edge colour of its own
    // (demo::text_field), and the surface's info claims the edge.
    let styled = taken.builder;
    let filled = taken.fill;
    let info = if taken.surface {
        info
    } else {
        info.color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/input.rs:817",
        ))
    };
    let info = if filled {
        info
    } else {
        info.color(input_background(t))
    };
    // Input takes only the fill from input_style and drops its foreground
    // (input/input.rs:715), so the text is the colour of the root: the one
    // geometry::input refines it with, or else the colour the showcase sets
    // on its window.
    let info = if field == InputField::Refined && styled {
        info.instance("text", "input.text(NONE).color, which geometry::input refines the root with: Input sets no text colour of its own and the editor paints the field's text in the root's (input/input.rs, Input::render; gpui-base/input/base/element.rs, prepaint)")
    } else {
        info.color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
    };
    let info = info.color(claim(
        "placeholder",
        "muted_foreground",
        t.muted_foreground,
        "gpui-component/input/input.rs:579",
    ));
    // geometry::input carries border.corner_radius, which the refinement
    // applies after upstream's own (input/input.rs:801, then :808).
    let info = if field == InputField::HeightOnly || !styled {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    } else {
        info
    };
    let info = info.config("focus_ring", format!("{}", t.focus_ring));
    let info = if filled {
        info
    } else {
        info.not_themeable("fill", "input_background(): the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs); geometry::input_fill gives a refined field the platform's")
    };
    let info = if field == InputField::Refined && styled {
        info
    } else {
        info.not_themeable("text colour","none of its own: Input takes only the fill from input_style and drops its foreground (input/input.rs, Input::render), so the text takes the colour the showcase sets on its window; geometry::input gives a refined field input.text(NONE).color")
    };
    let info = info
        .not_themeable("focus ring", "the connector uses the platform's focus_ring_width and focus_ring_keyboard_only only as a switch: upstream drops the ring where Theme::focus_ring is off, which the connector leaves off where the width is 0 or the theme says keyboard only, there but for a focus that followed a key press in a window under Root (styled.rs, FocusableExt::focus_ring_style), and draws it 3px wide with the ring colour's alpha set to 0.5 where it is on (styled.rs, FOCUS_RING_WIDTH), so the platform's width itself is Tier U. The ring is `ring` with that alpha, shown on the InputGroup's info; the OtpInput and Select show `ring` only as their focused border");
    let info = if filled {
        info
    } else {
        info.not_themeable("disabled fill", "input_style's Oklab mix of 80% input and 20% transparent, then faded to half alpha, because a disabled Input fades its fill again (input/input.rs, Input::render) -- a literal pair, and not muted")
    };
    let info = if filled {
        info
    } else {
        info.not_themeable("disabled text colour", "the window's text colour at half alpha: the editor dims its own text when disabled, in its paint preparation (gpui-base/input/base/element.rs, prepaint); geometry::input_fill gives a refined disabled field input.text(DISABLED).color to dim")
    };
    let info = info
        .not_themeable("placeholder colour", "Tier U: input.placeholder_color is modelled from each platform's own placeholder colour -- inheritance-rules.toml lists falling back to muted_color as wrong -- but Input hands its editor the shared muted_foreground on every render and takes no colour of its own (input/input.rs, Input::render)");
    match field {
        InputField::Refined if styled => info,
        InputField::Refined => info.instance(
            "padding",
            "upstream's own input_px and input_py for Size::Medium (input/input.rs, Input::render): geometry::input does not refine it (its geometry line says why), so the theme gives it no padding",
        ),
        InputField::HeightOnly if styled => info
            .instance(
                "height",
                "the height rule of the field above without the rest of its refinement: what geometry::input_height is for. At a text scale of 1 or less both are input.field(NONE).min_size[1] tall; above 1 each grows around its own text and padding, and this one's are upstream's text_sm and input_py for Size::Medium (input/input.rs, Input::render), not the platform's, so it no longer lines up with the field above",
            )
            .instance(
                "padding",
                "upstream's own input_px and input_py for Size::Medium (input/input.rs, Input::render): geometry::input_height carries no padding",
            ),
        InputField::HeightOnly => info.instance(
            "height",
            "upstream's own for Size::Medium: no 0.8.0 theme is installed, so geometry::input_height has no height to give it",
        ),
    }
}

/// A `Textarea`, with what it takes of the installed 0.8.0 theme, `taken`:
/// `geometry::text_area`, and the surface of its own field it is drawn over
/// (`demo::text_area_surface`). Its geometry line is recorded where
/// `demo::surfaced_textarea` applies the builder, and its surface's lines
/// where it draws the surface.
pub fn textarea(t: &Theme, taken: FieldTaken) -> WidgetInfo {
    let info = WidgetInfo::new("Textarea");
    // geometry::text_area letters the field in input.text(NONE).color, over
    // the window's it inherits (a Textarea renders as an Input,
    // input/textarea.rs:221, which drops input_style's foreground,
    // input/input.rs:715): its geometry line names it. A field over the
    // showcase's surface has no fill or edge colour of its own, and the
    // surface's info claims the edge.
    let info = if taken.fill {
        info
    } else {
        info.color(input_background(t))
    };
    let info = if taken.builder {
        info
    } else {
        info.color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
    };
    let info = if taken.surface {
        info
    } else {
        info.color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/input.rs:817",
        ))
    };
    info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/input/input.rs:792",
        ))
        .instance("row height", "typography.line_height_multiplier times the text size: geometry::text_area carries the platform's line height, which Input applies after its own 1.25rem (input/input.rs, Input::render: line_height then refine_style), and each row is the window's line height (gpui-base/input/base/element.rs, TextElement::request_layout)")
        .instance("height", "the showcase's own 90px, set after geometry::text_area, which sets none")
        .instance("padding", "the sides text_area.field(NONE) states, the multi-line field's own padding (docs/platform-facts.md §2.29), where it reaches: upstream pads a Textarea's editor in render by its Size's input padding (input/input.rs, Input::render; sizing.rs, input_px and input_py), Size::Medium's 10 across and 8 down here; the Size is the one seam (input/textarea.rs, Textarea::with_size), and it moves every side together by a fixed step, so no stated side reaches the editor through it, and geometry::text_area pads the root by the rest of each stated side; a stated side below the editor's stays the editor's, and an unstated side is upstream's")
        .instance("refinement", "geometry::text_area: input.text(NONE)'s font, and text_area.field(NONE)'s padding, corner radius and stroke width (a Textarea renders as an Input, input/textarea.rs, Textarea::into_input)")
}

/// A `Textarea` as tall as the rows its state shows: [`textarea`]'s, with
/// the height its rows give it.
pub fn rows_textarea(t: &Theme, taken: FieldTaken) -> WidgetInfo {
    let mut info = textarea(t, taken);
    info.instance.retain(|note| note.what != "height");
    info.instance("height", "the rows its state was built to show, three, each the row height above, the three rounded up to a whole device pixel: geometry::text_area sets no height, so the state's rows size it (gpui-base/input/base/state.rs, InputBaseState::auto_grow; gpui-base/input/base/element.rs, ceil_to_device_pixel)")
}

/// The three `InputGroup`s, which report as one. Their geometry lines are
/// recorded where `demo::input_groups` applies the builders.
pub fn input_groups(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("InputGroup")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/group.rs:328",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/input/group.rs:326",
        ));
    let info = if t.focus_ring {
        info.color(claim(
            "focus ring, ring at 50%",
            "ring",
            t.ring.opacity(0.5),
            "gpui-component/input/group.rs:326",
        ))
    } else {
        info.config("focus ring", NO_BAND)
    };
    info.color(claim(
            "addon text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/input/group.rs:415",
        ))
        .color(claim(
            "addon button hover",
            "secondary_hover",
            t.secondary_hover,
            "native-theme-gpui/variants.rs:63",
        ))
        .not_themeable("addon padding", "inner (Tier U)")
        .not_themeable("addon button", "native_theme_gpui::variants::ghost_button: flat idle, hover = secondary_hover (the platform's button.hover_background). Upstream's own in-group ghost would hover with muted (input/group.rs, InputGroupButton::render_in_group)")
        .instance("groups", "a Search icon of the chosen icon theme before the field, a Copy button after it, and a note under a textarea. geometry::input refines the two single-line groups; the textarea group keeps upstream's frame")
        .instance("padding", "none from geometry::input: the showcase leaves out its padding sides, because the refinement lands on the group's frame and the Input inside it keeps its own padding (input/group.rs, render_control) -- padding the frame too would inset the field twice")
        .instance("copy", "puts the second group's text on the clipboard and says so in a notification")
}

/// A `NumberInput`. Its geometry line is recorded where `demo::number_input`
/// applies the builder. `unstated` is the home an installed 0.8.0 theme
/// leaves without a value gpui can take, which the field the showcase draws
/// in its place needs (`demo::number_input`); `None` where no 0.8.0 theme
/// is installed.
pub fn number_input(t: &Theme, unstated: Option<&str>) -> WidgetInfo {
    let info = match unstated {
        Some(home) => WidgetInfo::new("NumberInput").config("drawn", format!("by gpui-component: the theme states no value gpui can take at {home}, which the field the showcase draws needs")),
        None => WidgetInfo::new("NumberInput"),
    };
    info
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/number_input.rs:141",
        ))
        .color(input_background(t))
        // The digits are the inner Input's, which drops input_style's
        // foreground (input/input.rs:715).
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .instance("padding", "none from geometry::input: the showcase leaves out its padding sides, because the refinement lands on the frame round the buttons (input/number_input.rs, NumberInput::render) and the Input inside it keeps upstream's own padding")
        .not_themeable("step buttons", "hardcoded +/- icons; the Size enum sets their min width (input/number_input.rs, NumberInput::render: min_w_6 / min_w_8), not the field's height")
        .not_themeable("own icons", super::own_icons("Minus and Plus on its step buttons (input/number_input.rs, NumberInput)"))
}

/// The size of the Number input's step glyphs under a native theme
/// (M9-177): `icons.sizes.small`, or where a step is shorter, the step's
/// height, so no glyph overflows its step. The steps share the field's
/// stated height inside its edge (`geometry::input`:
/// `input.field(NONE).min_size[1]` and its stroke width).
/// `demo::number_input` draws the glyphs at [`StepGlyph::side`] and
/// [`native_number_input`] says so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct StepGlyph {
    /// `icons.sizes.small`.
    pub(crate) small: f32,
    /// A step's height: half `input.field(NONE).min_size[1]` inside the
    /// frame's two edges.
    pub(crate) step: f32,
}

impl StepGlyph {
    /// The glyphs' sizes under `r`, or the home `r` leaves without a value
    /// they need.
    pub(crate) fn of(r: &NextTheme) -> Result<Self, &'static str> {
        let field = r.input.field(WidgetState::NONE);
        let height = field.min_size[1].ok_or("input.field(NONE).min_size[1]")?;
        let line = stroke_width_of(&field).ok_or("input.field(NONE).stroke.width")?;
        Ok(Self {
            small: r.icons.sizes.small.ok_or("icons.sizes.small")?,
            step: (height - line - line).max(0.0) / 2.0,
        })
    }

    /// The side each glyph is drawn at.
    pub(crate) fn side(self) -> f32 {
        self.small.min(self.step)
    }

    /// Whether the glyphs are at `icons.sizes.small`
    /// (`geometry::icon_size_small`), a step being at least that tall.
    pub(crate) fn at_icon_size_small(self) -> bool {
        self.small <= self.step
    }
}

/// The number field the showcase draws as `r`'s `input` states a text
/// field (`demo::number_input` under a 0.8.0 theme): its own frame round
/// gpui-base's NumberInput, `focused` or not, its line `width` wide and
/// rounded by `radius`, its step buttons showing `up` and `down`, of the
/// chosen icon theme, at `glyph`'s side; focused, where `r` states no
/// focused edge, edged in `t`'s `ring`, as upstream's (`demo::number_edge`).
/// Its geometry lines are recorded where `demo::number_input` applies the
/// builders.
#[allow(clippy::too_many_arguments)]
pub fn native_number_input(
    r: &NextTheme,
    t: &Theme,
    focused: bool,
    up: &SampleIcon,
    down: &SampleIcon,
    glyph: StepGlyph,
    width: f32,
    radius: f32,
) -> WidgetInfo {
    let info = WidgetInfo::new("NumberInput")
        .variant(if focused {
            "drawn by the showcase, focused"
        } else {
            "drawn by the showcase"
        })
        .colors(stated_claim(
            "bg",
            "input.field(NONE).fill",
            fill_of(&r.input.field(WidgetState::NONE)),
            "showcase",
        ));
    // The frame takes geometry::input's stroke width; a 0 width draws no
    // edge in any state. Where the theme states no focused edge, the
    // focused frame is edged in upstream's ring, as gpui-component's
    // NumberInput edges its own: the toolkit changes the edge colour between
    // rest and focus (spec §10, rule 0).
    let drawn = Some(width);
    let focus = state_stroke_color_of(&r.input.field(WidgetState::FOCUSED));
    let info = if width <= 0.0 {
        if matches!(r.input.field(WidgetState::NONE).stroke, Stroke::None) {
            info.config("edge", "none: input.field(NONE).stroke is none")
        } else {
            info.config("edge", "none: input.field(NONE).stroke.width is 0")
        }
    } else if focused {
        info.colors(
            edge_claim(
                "edge, focused",
                "input.field(FOCUSED).stroke.color",
                state_stroke_color_of(&r.input.field(WidgetState::FOCUSED)),
                drawn,
                "showcase",
            )
            .or(edge_claim(
                "edge, focused: no focused edge stated, so upstream's",
                "ring",
                focus.is_none().then_some(t.ring),
                drawn,
                "gpui-component/input/number_input.rs:231",
            )),
        )
        .config(
            "hover",
            "none while focused: the frame keeps its focused edge under the pointer",
        )
    } else {
        info.colors(edge_claim(
            "edge",
            "input.field(NONE).stroke.color",
            stroke_color_of(&r.input.field(WidgetState::NONE)),
            drawn,
            "showcase",
        ))
        .colors(edge_claim(
            "hovered edge",
            "input.field(HOVER).stroke.color",
            state_stroke_color_of(&r.input.field(WidgetState::HOVER)),
            drawn,
            "showcase",
        ))
        .colors(
            edge_claim(
                "focused edge",
                "input.field(FOCUSED).stroke.color",
                state_stroke_color_of(&r.input.field(WidgetState::FOCUSED)),
                drawn,
                "showcase",
            )
            .or(edge_claim(
                "focused edge, none stated: upstream's",
                "ring",
                focus.is_none().then_some(t.ring),
                drawn,
                "gpui-component/input/number_input.rs:231",
            )),
        )
    };
    // The step buttons take upstream's colours, through the homes its
    // tokens carry under a 0.8.0 theme: secondary_foreground is the button
    // label's colour, and input the field's edge colour
    // (input/number_input.rs, NumberInput::render). A step whose chevron the
    // icon theme lacks shows none, and a raster chevron keeps its own
    // colours (`up.note`, `down.note`).
    let svg = |icon: &SampleIcon| icon.shown() && !icon.is_raster();
    let info = if svg(up) || svg(down) {
        info.colors(stated_claim(
            "step arrows",
            "button.label(NONE).color",
            text_color_of(&r.button.label(WidgetState::NONE)),
            "showcase",
        ))
    } else {
        info
    };
    let info = info
        .colors(stated_claim(
            "step button hovered, the edge colour at 40%",
            "input.field(NONE).stroke.color",
            stroke_color_of(&r.input.field(WidgetState::NONE))
                .map(|edge| edge.opacity(NUMBER_STEP_HOVER)),
            "showcase",
        ))
        .colors(stated_claim(
            "step button pressed, the edge colour at 60%",
            "input.field(NONE).stroke.color",
            stroke_color_of(&r.input.field(WidgetState::NONE))
                .map(|edge| edge.opacity(NUMBER_STEP_ACTIVE)),
            "showcase",
        ));
    let side = |name: &str, v: Option<f32>| match v {
        Some(v) => format!("{name} {}px", px_text(v)),
        None => format!("{name} upstream's"),
    };
    let [top, right, bottom, left] = padding_of(&r.input.field(WidgetState::NONE));
    let sides = [
        side("top", top),
        side("right", right),
        side("bottom", bottom),
        side("left", left),
    ]
    .join(", ");
    info.config("why drawn", "upstream's NumberInput fills its frame with input_background and edges it with the input token, ring while focused, and no hover edge (input/number_input.rs, NumberInput::render), so input.field(NONE).fill and the hovered and focused field's stroke colours would have no receiver there: the showcase draws the frame round gpui-base's NumberInput, the spin button upstream composes")
        .config("padding", format!("geometry::input's padding, the sides input.field(NONE) states, on the value inside the step buttons, {sides}: an unstated side keeps upstream's Size::Medium padding, which the refinement lands after (input/input.rs, Input::render: input_px and input_py, then refine_style); the frame round the buttons takes none"))
        .config("frame", format!("geometry::input's height, radius {}px and edge width {}px, on the frame round the value and the buttons", px_text(radius), px_text(width)))
        .instance("text", "input.text(NONE).color, which geometry::input refines the value with: Input sets no text colour of its own and the editor paints the field's text in the root's (input/input.rs, Input::render; gpui-base/input/base/element.rs, prepaint)")
        .instance("focus ring", "none: the value is an Input without its appearance, which carries upstream's ring (input/input.rs, Input::render), and the frame marks focus by its edge alone")
        .not_themeable("step buttons", "upstream's: transparent at rest, as wide as upstream's at the field's Size (input/number_input.rs, NumberInput::render: min_w_8), each half the field's height, stacked at its right end (gpui-base/number_input.rs, controls_right): the theme states no spin-box layout")
        .instance("step glyphs", format!(
            "{}px: the smaller of icons.sizes.small, {}px, and a step's height, half input.field(NONE).min_size[1] inside the frame's edge, {}px, so no glyph overflows its step (demo::number_input)",
            px_text(glyph.side()),
            px_text(glyph.small),
            px_text(glyph.step)
        ))
        .instance("increment step", format!("ChevronUp: {}", up.note("the button shows no glyph")))
        .instance("decrement step", format!("ChevronDown: {}", down.note("the button shows no glyph")))
}

/// A two-state control's states, on and off, and what a click on it does
/// while it is enabled.
struct Toggled {
    on: &'static str,
    off: &'static str,
    click: &'static str,
}

/// A Checkbox's, upstream's and the theme-drawn one's alike.
const CHECKBOX: Toggled = Toggled {
    on: "checked",
    off: "unchecked",
    click: "checks or unchecks it; the showcase keeps the state",
};

/// A Switch's, upstream's and the theme-drawn one's alike.
const SWITCH: Toggled = Toggled {
    on: "on",
    off: "off",
    click: "turns it on or off; the showcase keeps the state",
};

/// `info` named by the state of a `control` -- `checked` or not, `disabled`
/// or not -- and given what a click does.
fn toggled(info: WidgetInfo, control: Toggled, checked: bool, disabled: bool) -> WidgetInfo {
    let state = if checked { control.on } else { control.off };
    if disabled {
        info.variant(format!("{state}, disabled")).instance(
            "click",
            "none: it is disabled, and the showcase gives it no handler",
        )
    } else {
        info.variant(state).instance("click", control.click)
    }
}

/// A `Checkbox` reading `label`, `checked` or not, `disabled` or not. Its
/// geometry line is recorded where `demo::checkbox` applies the builder.
pub fn checkbox(t: &Theme, label: &'static str, checked: bool, disabled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Checkbox").not_themeable(
        "own icons",
        super::own_icons(
            "the check mark, Check, drawn while it is checked (checkbox.rs, checkbox_check_icon)",
        ),
    );
    // The disabled style is resolved after the checked one, so it wins
    // (gpui-base/checkbox.rs:308-322).
    let info = match (checked, disabled) {
        (true, false) => info
            .color(claim(
                "checked bg",
                "primary",
                t.primary,
                "gpui-component/checkbox.rs:352",
            ))
            .color(claim(
                "checked border",
                "primary",
                t.primary,
                "gpui-component/checkbox.rs:285",
            ))
            .color(claim(
                "checkmark",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/checkbox.rs:233",
            )),
        (false, false) => info.color(input_background(t)).color(claim(
            "unchecked border",
            "input",
            t.input,
            "gpui-component/checkbox.rs:284",
        )),
        (true, true) => info
            .color(claim(
                "checked bg and border, at 50%",
                "primary",
                t.primary.opacity(0.5),
                "gpui-component/checkbox.rs:285-287",
            ))
            .color(claim(
                "checkmark, at 50%",
                "primary_foreground",
                t.primary_foreground.opacity(0.5),
                "gpui-component/checkbox.rs:231",
            )),
        (false, true) => info.color(input_background(t)).color(claim(
            "unchecked border, at 50%",
            "input",
            t.input.opacity(0.5),
            "gpui-component/checkbox.rs:284-289",
        )),
    };
    let info = if disabled {
        info.color(claim(
            "label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/checkbox.rs:392",
        ))
    } else {
        info.color(claim(
            "label",
            "foreground",
            t.foreground,
            "gpui-component/checkbox.rs:390",
        ))
    };
    let info = info
        .config("indicator radius", format!("radius, capped at 4px: {}px", t.radius.as_f32().min(4.0)))
        .not_themeable("font colour", "carried as size and weight only. Upstream wraps a Checkbox label in a div that sets foreground itself and re-sets muted_foreground there when disabled (gpui-component/checkbox.rs, Checkbox::render), and the disabled hook applies muted_foreground before this refinement, so a carried colour would never reach the label and would displace the disabled colour of custom children (native-theme-gpui/geometry.rs, geometry::checkbox)")
        .not_themeable("unchecked fill", "input_background(), as an Input's: the window background in light mode, input faded toward transparent in dark (gpui-component/checkbox.rs, Checkbox::render)")
        .not_themeable("indicator size", "rems(0.75 / 0.875 / 1 / 1.125) per Size (gpui-component/checkbox.rs, Checkbox::render indicator_size), so the box already scales with the platform font -- the rem is Theme::font_size. What has no route is checkbox.indicator_width, which the model states in absolute px: Size::Size falls into the same catch-all arm as Medium. Tier U for the px, not for the scaling")
        .instance("label", label);
    toggled(info, CHECKBOX, checked, disabled)
}

/// A horizontal `RadioGroup` of Radios reading `labels`, the one at
/// `selected` selected. Its geometry line is recorded where
/// `demo::radio_group` applies the builder.
pub fn radio_group(t: &Theme, labels: &[&'static str], selected: Option<usize>) -> WidgetInfo {
    let selected = selected
        .and_then(|ix| labels.get(ix).copied())
        .unwrap_or("none");
    WidgetInfo::new("RadioGroup")
        .variant("horizontal")
        .color(claim(
            "selected fill and border",
            "primary",
            t.primary,
            "gpui-component/radio.rs:194",
        ))
        .color(claim(
            "unselected border",
            "input",
            t.input,
            "gpui-component/radio.rs:196",
        ))
        // The unselected Radios' fill: Radio::render takes
        // input_background() for an unchecked one (radio.rs:243).
        .color(input_background(t))
        .color(claim(
            "upstream label",
            "foreground",
            t.foreground,
            "gpui-component/radio.rs:222",
        ))
        .not_themeable("unselected fill", "input_background(), as an Input's -- not the input at 50% upstream computes and never paints. Disabled halves the border and a checked fill, never an unchecked one (gpui-component/radio.rs, Radio::render)")
        .not_themeable("corner radius", "a circle, through radius_full() -- and square where the theme's radius is 0, since radius_full() follows it (gpui-component/styled.rs, rounded_full_style). The radius * 0.5 is the row's, which only the focus ring shows (gpui-component/radio.rs, Radio::render)")
        .not_themeable("indicator size", "rems per the Size enum, so it follows the root font size rather than the theme's checkbox metrics (gpui-component/radio.rs, Radio::render indicator_size)")
        .instance("radios", format!("{}. RadioGroup::child takes an Into<Radio>, not an element a target could wrap, so they report through the group (radio.rs, RadioGroup::child)", labels.join(", ")))
        .instance("selected", selected)
}

/// A column of Radios reading `labels`, the one at `selected` selected,
/// built Radio by Radio: what a `RadioGroup` paints on each (`radio_group`),
/// in the showcase's own column. Its geometry lines are recorded where
/// `demo::radio_column` applies the builders.
pub fn radio_column(t: &Theme, labels: &[&'static str], selected: Option<usize>) -> WidgetInfo {
    let mut info = radio_group(t, labels, selected).variant("vertical");
    info.instance.retain(|note| note.what != "radios");
    info.instance("radios", format!("{}, each built with Radio::new and reporting through this column: a RadioGroup lays its Radios out gap_3 apart in a child of its own, which its style does not reach (radio.rs, RadioGroup), so the showcase lays them out widget_gap apart itself", labels.join(", ")))
}

/// A `Switch` reading `label`, `checked` or not, `disabled` or not, its
/// checked track handed `native`'s `switch.track(CHECKED).fill` through
/// `next::variants::switch_color` where a 0.8.0 theme it maps is installed.
pub fn switch(
    t: &Theme,
    label: &'static str,
    checked: bool,
    disabled: bool,
    native: Option<&NextTheme>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Switch");
    // The theme whose checked track the showcase hands `Switch::color`
    // (`demo::switch`): one `variants::switch_color` maps.
    let native = native.filter(|r| native_theme_gpui::next::variants::switch_color(r).is_ok());
    // A disabled Switch fades its track alone, to half alpha (switch.rs:190);
    // the disabled style is resolved after the checked one (gpui-base
    // switch.rs:117-127). The checked track is the colour `Switch::color`
    // was handed, primary where it was handed none (gpui-component
    // switch.rs:181-184).
    let info = match (checked, disabled, native) {
        (true, false, Some(r)) => info.colors(stated_claim(
            "on track",
            "switch.track(CHECKED).fill",
            fill_of(&r.switch.track(WidgetState::CHECKED)),
            "showcase",
        )),
        (true, true, Some(r)) => info.colors(stated_claim(
            "on track, at 50%",
            "switch.track(CHECKED).fill",
            fill_of(&r.switch.track(WidgetState::CHECKED)).map(|fill| fill.opacity(0.5)),
            "showcase",
        )),
        _ => info,
    };
    let info = match (checked, disabled) {
        (true, _) if native.is_some() => info,
        (true, false) => info.color(claim(
            "on track",
            "primary",
            t.primary,
            "gpui-component/switch.rs:186",
        )),
        (false, false) => info.color(claim(
            "off track",
            "switch",
            t.switch,
            "gpui-component/switch.rs:187",
        )),
        (true, true) => info.color(claim(
            "on track, at 50%",
            "primary",
            t.primary.opacity(0.5),
            "gpui-component/switch.rs:184-190",
        )),
        (false, true) => info.color(claim(
            "off track, at 50%",
            "switch",
            t.switch.opacity(0.5),
            "gpui-component/switch.rs:185-190",
        )),
    };
    let info = info.color(claim(
        "thumb",
        "switch_thumb",
        t.switch_thumb,
        "gpui-component/switch.rs:193",
    ));
    // 0.7.0: the track draws the focus ring while focused (switch.rs,
    // `Switch::render`); a disabled switch takes no focus.
    let info = if disabled {
        info
    } else if t.focus_ring {
        info.color(claim(
            "focus ring, ring at 50% alpha",
            "ring",
            t.ring.alpha(0.5),
            "gpui-component/styled.rs:261",
        ))
    } else {
        // With the band off the bordered track (switch.rs:278), focused through
        // focus_ring_style (:289-290), has its border tinted (styled.rs:269-270).
        info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/styled.rs:270",
        ))
        .config("focus ring", NO_BAND)
    };
    let info = if disabled {
        info.color(claim(
            "disabled label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/switch.rs:194",
        ))
    } else {
        info
    };
    // The track takes the theme's radius only under 4px; from 4px up it is
    // rounded by its own height (switch.rs:205-209).
    let info = if t.radius.as_f32() < 4.0 {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    } else {
        info.config("border-radius", format!("fully round: the theme's radius, {}px, is 4px or more, so the track is rounded by its own height instead", t.radius.as_f32()))
    };
    let info = match native {
        Some(_) => info.config("on track", "switch.track(CHECKED).fill, through variants::switch_color, which the showcase hands Switch::color, a per-instance receiver: ThemeColor has no field for it, so the connector cannot install it (gpui-component/switch.rs, Switch::color)"),
        None => info.not_themeable("on track", "primary: no installed 0.8.0 theme states switch.track(CHECKED).fill in a form gpui can take, so the showcase has no checked track to hand Switch::color (gpui-component/switch.rs, Switch::color)"),
    };
    let info = info
        .not_themeable("disabled", "the track at 50%, never the thumb: gpui multiplies each primitive's alpha rather than fading the subtree as a group, so fading both would let the track show through (gpui-component/switch.rs, Switch::render disabled_bg)")
        .not_themeable("size", "Tier U: track and thumb are px literals per Size, on children of the wrapper the refinement lands on (gpui-component/switch.rs, Switch::render), while the model states switch.track_width, track_height and thumb_diameter")
        .not_themeable("corner radius", "fully round unless the theme's radius is under 4px, in which case the theme's is used (gpui-component/switch.rs, Switch::render radius). switch.track_radius is modelled and has no receiver")
        .not_themeable("animation timing", "reads the theme's spring_move (gpui-component/switch.rs, Switch), the same writable Theme::motion the Accordion and Collapsible use; the connector leaves it at its default because native-theme models no motion")
        .instance("label", label);
    toggled(info, SWITCH, checked, disabled)
}

/// A `Slider`.
pub fn slider(t: &Theme, vertical: bool, range: bool) -> WidgetInfo {
    slider_shape(WidgetInfo::new("Slider"), vertical, range)
        .color(claim(
            "track",
            "slider_bar",
            t.slider_bar,
            "gpui-component/slider.rs:198",
        ))
        .color(claim(
            "thumb",
            "slider_thumb",
            t.slider_thumb,
            "gpui-component/slider.rs:204",
        ))
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/slider.rs:325",
        ))
        .not_themeable("track height", "Tier U, not an absence: slider.track_height is modelled and carried. Upstream sets h_1p5() on SliderIndicator, a child of the element the refinement lands on (gpui-component/slider.rs, Slider::render)")
        .not_themeable("thumb size", "Tier U, not an absence: slider.thumb_diameter is modelled and carried. The thumb is built by a closure inside the indicator, deeper still than the track (gpui-component/slider.rs, Slider::render)")
        .not_themeable("focus ring", "a ring round the thumb in ring, the theme's focus ring colour, with its alpha set to 0.5, 3px wide, two literals the theme's defaults.focus_ring_width and focus_ring_offset do not reach (gpui-component/slider.rs, THUMB_RING_WIDTH, THUMB_RING_OPACITY), grown on a spring while the slider has the focus or the pointer is over the thumb or drags it; the keys move the end thumb, which carries it (gpui-component/slider.rs:63, focused; :228, ring_color; :233-238, end_ring)")
}

/// A slider's variant: a range state's two thumbs, a vertical one's axis.
fn slider_shape(info: WidgetInfo, vertical: bool, range: bool) -> WidgetInfo {
    let info = match (vertical, range) {
        (true, true) => info.variant("vertical range"),
        (true, false) => info.variant("vertical"),
        (false, true) => info.variant("range"),
        (false, false) => info,
    };
    let info = if range {
        info.instance("range", "a thumb at each end of the range and the fill between them; a press moves the nearer thumb (gpui-base/slider.rs, SliderTrack)")
    } else {
        info
    };
    if vertical {
        info.instance("axis", "vertical: the minimum at the bottom, as gpui-base maps the pointer (slider.rs, SliderState::update_value_by_position)")
    } else {
        info
    }
}

/// The note an upstream control's info takes in place of the band claim
/// where `Theme::focus_ring` is off.
const NO_BAND: &str = "no band for this focus: Theme::focus_ring is off where the theme's ring has no width, and, where the theme says the platform shows its ring for keyboard focus only, for a focus that did not follow a key press (native-theme-gpui/colors.rs, focus_band_on; native-theme-gpui/lib.rs, FocusBand)";

/// Where the theme-drawn widgets read what they paint.
#[cfg(feature = "widgets")]
const DRAWN_BY: &str = "native_theme_gpui::widgets, on gpui-base's headless primitive, every part from the theme (docs/archive/todo_gpui-widgets-spec.md)";

/// What the showcase draws its theme-drawn controls on, which a disabled one
/// is faded over as one (`CheckboxLook::faded_over`,
/// `SwitchLook::faded_over`): the window's `colors.window.background`, the
/// widgets' own where no backdrop is handed them, and the showcase hands
/// none; `None` where the theme states none and the widget draws
/// gpui-component's own control in its place
/// (native-theme-gpui/widgets/mod.rs, `ThemeSource::background`).
#[cfg(feature = "widgets")]
fn next_ground(r: &NextTheme) -> Option<Hsla> {
    r.colors.window.background.map(next_stated)
}

/// The home a theme-drawn checkbox's or radio's label font is read at,
/// `home`, where the theme gives no font gpui can letter it in there: the
/// widget then draws gpui-component's own control in its place
/// (native-theme-gpui/widgets/mod.rs, `LabelFont::next`, through the
/// connector's own check, `next::label_font_unmapped`); `None` where it
/// gives one.
#[cfg(feature = "widgets")]
fn unstated_label_font(text: &TextStyle, home: &'static str) -> Option<&'static str> {
    native_theme_gpui::next::label_font_unmapped(text, home).map(|unstated| unstated.home())
}

/// The `label font` line of a theme-drawn switch: each property of `font`
/// (`SwitchLabelFont::next_of`) at `switch.label(NONE)` where the theme
/// states it, else gpui-component's own, its switch's label setting no
/// family, weight or slant and the size `input_text_size(Size::Medium)`.
#[cfg(feature = "widgets")]
fn switch_label_font_text(font: Option<&SwitchLabelFont>) -> String {
    let Some(font) = font else {
        return "none gpui can take at switch.label(NONE)".to_owned();
    };
    let family = font.family.as_ref().map_or_else(
        || "the window's family (none stated at switch.label(NONE).family)".to_owned(),
        |family| format!("switch.label(NONE).family {family}"),
    );
    let size = font.size.map_or_else(
        || "gpui-component's own size, text_sm, input_text_size(Size::Medium) (none stated at switch.label(NONE).size_px)".to_owned(),
        |size| format!("switch.label(NONE).size_px {}", length_text(size)),
    );
    let weight = font.weight.map_or_else(
        || "the window's weight (none stated at switch.label(NONE).weight)".to_owned(),
        |weight| format!("switch.label(NONE).weight {}", weight.0),
    );
    let style = font.style.map_or_else(
        || "the window's slant (none stated at switch.label(NONE).style)".to_owned(),
        |style| format!("switch.label(NONE).style {style:?}"),
    );
    format!(
        "{family}, {size}, {weight}, {style}, one line typography.line_height_multiplier {} tall (native-theme-gpui/widgets/switch.rs, SwitchLabelFont::next_of; gpui-component/switch.rs, Switch::render)",
        font.line_height
    )
}

/// Whether a theme-drawn switch, `checked` or not, is hovered in a colour
/// of its own, as `SwitchLook::next_of` reads it: where a rule states the
/// hovered track's fill, as the query with that flag alone says
/// (`switch.track(HOVER).from_state`, spec §2), an unchecked one in
/// `switch.track(HOVER).fill`, a checked one in
/// `switch.track(HOVER|CHECKED).fill`, whatever its value.
#[cfg(feature = "widgets")]
pub(crate) fn switch_hover_stated(r: &NextTheme, checked: bool) -> bool {
    let hover = r.switch.track(WidgetState::HOVER);
    if checked {
        hover.from_state.contains(PartStated::FILL)
    } else {
        state_fill_of(&hover).is_some()
    }
}

/// What a theme-drawn checkbox or radio is drawn from: its `look`, where the
/// theme states it, over `ground`, the window's colour, with its label
/// font stated (`unstated_font` none); else the first home the theme leaves
/// without a value gpui can take, the look's, the ground's or the label
/// font's, and the widget draws gpui-component's own control in its place
/// ([`drawn_by_upstream`]).
#[cfg(feature = "widgets")]
pub(crate) fn drawn_or_upstream<L>(
    look: Result<L, &'static str>,
    ground: Option<Hsla>,
    unstated_font: Option<&'static str>,
) -> Result<(L, Hsla), &'static str> {
    let look = look?;
    let ground = ground.ok_or("colors.window.background")?;
    match unstated_font {
        Some(home) => Err(home),
        None => Ok((look, ground)),
    }
}

/// The `drawn by` line of a theme-drawn `control` that draws
/// gpui-component's own in its place, the theme stating no value gpui can
/// take at `home` (native-theme-gpui/widgets/checkbox.rs, `Parts::render`).
#[cfg(feature = "widgets")]
pub(crate) fn drawn_by_upstream(info: WidgetInfo, control: &str, home: &str) -> WidgetInfo {
    info.config(
        "drawn by",
        format!(
            "gpui-component's own {control}, in upstream's colours and sizes: the theme states no value gpui can take at {home}, so widgets::{control} draws upstream's in its place (native-theme-gpui/widgets/checkbox.rs, Parts::render)"
        ),
    )
}

/// A length of a connector Look as Widget Info prints one.
#[cfg(feature = "widgets")]
fn length_text(length: Pixels) -> String {
    format!("{}px", px_text(f32::from(length)))
}

/// A theme-drawn `widgets::Checkbox` (spec §2.1 of
/// docs/archive/todo_gpui-widgets-spec.md) reading `label`, `checked` or not,
/// `disabled` or not, under the installed 0.8.0 theme `r`: what
/// `CheckboxLook::next_of` gives it, the one source the widget draws from,
/// faded as the widget fades a disabled one.
#[cfg(feature = "widgets")]
pub fn native_checkbox(
    r: &NextTheme,
    label: &'static str,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    drawn_checkbox(r, label, checked, disabled, false)
}

/// A theme-drawn `widgets::Checkbox` reading `label` in the mixed state,
/// `indeterminate(true)`, under `r`, with no handler: drawn as a checked one,
/// gpui-component's dash in place of the check (native-theme-gpui
/// widgets/checkbox.rs, `Checkbox::indeterminate`).
#[cfg(feature = "widgets")]
pub fn native_mixed_checkbox(r: &NextTheme, label: &'static str) -> WidgetInfo {
    drawn_checkbox(r, label, true, false, true)
}

/// What a theme-drawn `widgets::Checkbox`, `checked` or not, `disabled` or
/// not, is drawn from under `r` ([`drawn_or_upstream`]): its look and the
/// window's colour, or the first home the theme leaves without a value gpui
/// can take, and the widget draws gpui-component's own in its place.
#[cfg(feature = "widgets")]
pub(crate) fn checkbox_drawn(
    r: &NextTheme,
    checked: bool,
    disabled: bool,
) -> Result<(CheckboxLook, Hsla), &'static str> {
    drawn_or_upstream(
        CheckboxLook::next_of(r, checked, disabled).map_err(|unstated| unstated.home()),
        next_ground(r),
        unstated_label_font(&r.checkbox.label(WidgetState::NONE), "checkbox.label(NONE)"),
    )
}

/// [`native_checkbox`], `mixed` or not.
#[cfg(feature = "widgets")]
fn drawn_checkbox(
    r: &NextTheme,
    label: &'static str,
    checked: bool,
    disabled: bool,
    mixed: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("Checkbox");
    let text = r.checkbox.label(WidgetState::NONE);
    let (rest, ground) = match checkbox_drawn(r, checked, disabled) {
        Ok(drawn) => drawn,
        Err(home) => {
            let info = drawn_by_upstream(info, "Checkbox", home);
            return checkbox_state(info.instance("label", label), checked, disabled, mixed);
        }
    };
    let look = rest.faded_over(ground);
    let info = native_indicator(
        info.config("drawn by", DRAWN_BY),
        r,
        false,
        &look,
        checked,
        disabled,
    );
    let info = info
        .config(
            "indicator",
            format!(
                "checkbox.indicator(NONE).min_size[0]: {} square",
                length_text(look.indicator)
            ),
        )
        .config(
            "corner radius",
            format!(
                "checkbox.indicator(NONE).corners: {}",
                length_text(look.radius)
            ),
        )
        .config(
            "label gap",
            format!("checkbox.label_gap: {}", length_text(look.label_gap)),
        )
        .config(
            "label font",
            format!(
                "checkbox.label(NONE): {}",
                next_drawn_font(&text, LINE_AT_THE_MULTIPLIER)
            ),
        );
    let inside = if look.mark_padded {
        "checkbox.indicator(NONE).padding, and inside its stroke on a side the theme leaves unstated"
    } else {
        "its stroke"
    };
    let info = match (look.mark_stroke, mixed) {
        (Some(stroke), false) => info.config("check mark", format!(
            "the connector strokes upstream's check polyline at checkbox.checkmark(NONE).stroke_width, {}, in a {} square, the box inside {inside} (native-theme-gpui/widgets/checkbox.rs, stroked_check)",
            length_text(stroke),
            length_text(look.mark_size),
        )),
        (Some(stroke), true) => info.config("mixed mark", format!(
            "the connector strokes upstream's dash, the line of its Minus icon, at checkbox.checkmark(NONE).stroke_width, {}, in a {} square, the box inside {inside}, in the checked mark's colour (native-theme-gpui/widgets/checkbox.rs, MINUS_POINTS; stroked_check)",
            length_text(stroke),
            length_text(look.mark_size),
        )),
        (None, false) => info.not_themeable("check mark", super::own_icons("Check, the mark gpui-component's own checkbox draws (checkbox.rs, checkbox_check_icon), which widgets::Checkbox draws where the theme states no checkbox.checkmark(NONE).stroke_width, filling the box inside its stroke")),
        (None, true) => info.not_themeable("mixed mark", super::own_icons("Minus, the dash gpui-component's own checkbox draws in the mixed state (checkbox.rs, checkbox_check_icon), which widgets::Checkbox draws through the application's icon resolver where the theme states no checkbox.checkmark(NONE).stroke_width, filling the box inside its stroke, in the checked mark's colour")),
    };
    let info = if mixed {
        info.not_themeable("mixed look", "the checked state's fill, border and mark colour: the model states no look of its own for the mixed state, so widgets::Checkbox draws it as checked, with the dash in place of the check (native-theme-gpui/widgets/checkbox.rs, Checkbox::indeterminate)")
    } else {
        info
    };
    let info = if disabled {
        info.config("disabled", if rest.opacity < 1. {
            format!("checkbox.disabled_opacity {}: the whole control faded as one over the window's colors.window.background, so each colour above is composited over what it is drawn on and faded by it (native-theme-gpui/widgets/checkbox.rs, CheckboxLook::faded_over)", percent_text(rest.opacity))
        } else {
            format!("checkbox.disabled_opacity {}: no fade, the colours above as the theme states them (native-theme-gpui/widgets/checkbox.rs, CheckboxLook::faded_over)", percent_text(rest.opacity))
        })
    } else {
        info
    };
    // While focused the row draws the theme's focus ring; a disabled checkbox
    // takes no focus.
    let info = if disabled {
        info
    } else {
        next_focus_ring(info, r, "row")
    };
    checkbox_state(info.instance("label", label), checked, disabled, mixed)
}

/// `info` named by a checkbox's state, as [`toggled`] names it, or as the
/// mixed state, which the showcase gives no handler.
fn checkbox_state(info: WidgetInfo, checked: bool, disabled: bool, mixed: bool) -> WidgetInfo {
    if mixed {
        info.variant("mixed").instance(
            "click",
            "none: the showcase gives it no handler, so it stays mixed",
        )
    } else {
        toggled(info, CHECKBOX, checked, disabled)
    }
}

/// gpui-component's own `Checkbox` reading `label` in the mixed state,
/// `indeterminate(true)`, with no handler. Its geometry line is recorded
/// where `demo::mixed_checkbox` applies the builder.
pub fn mixed_checkbox(t: &Theme, label: &'static str) -> WidgetInfo {
    let info = WidgetInfo::new("Checkbox")
        .color(claim(
            "mixed bg",
            "primary",
            t.primary,
            "gpui-component/checkbox.rs:357",
        ))
        .color(claim(
            "mixed border",
            "primary",
            t.primary,
            "gpui-component/checkbox.rs:285",
        ))
        .color(claim(
            "dash",
            "primary_foreground",
            t.primary_foreground,
            "gpui-component/checkbox.rs:233",
        ))
        .color(claim(
            "label",
            "foreground",
            t.foreground,
            "gpui-component/checkbox.rs:390",
        ))
        .not_themeable("mixed look", "the checked state's: its border in primary and its fill in tokens.primary, which the connector fills from the same accent (gpui-component/checkbox.rs:354-358, indeterminate)")
        .not_themeable("mark", "IconName::Minus, a dash, in place of the check, asked of the application's icon resolver first, in primary_foreground (gpui-component/checkbox.rs:365-372, CheckboxState::Indeterminate; :249, resolve_icon_path; :230-234, primary_foreground)")
        .not_themeable("own icons", super::own_icons("the dash, Minus, drawn while it is mixed (checkbox.rs, checkbox_check_icon)"))
        .config("indicator radius", format!("radius, capped at 4px: {}px", t.radius.as_f32().min(4.0)))
        .instance("label", label);
    checkbox_state(info, true, false, true)
}

/// The focus ring a theme-drawn control draws round its `part` while focused
/// under the installed 0.8.0 theme, as `FocusRing::next_of` reads it
/// (native-theme-gpui/widgets/mod.rs): `colors.focus_ring`,
/// `focus_ring.stroke.width` wide, its outer edge `focus_ring.offset` plus
/// the width outside the part, drawn for a focus that followed a key press
/// only where `focus_ring.trigger` says keyboard only; inset at the edge it
/// is a band inside the part's edge, as none of these parts has a border of
/// its own.
#[cfg(feature = "widgets")]
fn next_focus_ring(info: WidgetInfo, r: &NextTheme, part: &str) -> WidgetInfo {
    let offset = r.focus_ring.offset;
    let width = native_theme_gpui::next::focus_ring_width(r)
        .ok()
        .filter(|width| width.is_finite() && offset.is_finite() && *width > 0.0);
    let Some(width) = width else {
        return info.config(
            "focus ring",
            "none: the theme's focus_ring.stroke.width is not positive, or is stated per side (native-theme-gpui/widgets/mod.rs, FocusRing::next_of)",
        );
    };
    let place = if offset == -width {
        format!(
            "inset at the {part}'s edge, a band inside it, the {part} having no border of its own to take the colour"
        )
    } else {
        format!(
            "its outer edge {}px outside the {part}'s, focus_ring.offset {}px plus the width, its corners grown as much",
            px_text(offset + width),
            px_text(offset)
        )
    };
    info.color(claim(
        "focus ring",
        "colors.focus_ring",
        next_stated(r.colors.focus_ring),
        "native-theme-gpui/widgets/mod.rs:363",
    ))
    .config(
        "focus ring",
        format!(
            "focus_ring.stroke.width {}px wide, {place}, painted over the {part}'s other parts (native-theme-gpui/widgets/mod.rs, FocusRing::draw){}",
            px_text(width),
            if r.focus_ring.trigger == Some(FocusTrigger::KeyboardOnly) {
                ", for a focus that followed a key press only (native-theme-gpui/widgets/mod.rs, ring_shows)"
            } else {
                ""
            }
        ),
    )
}

/// The indicator's colours for a checkbox, or a `radio`, in this state,
/// `look`'s, each claim naming the home `CheckboxLook::next_of` (or
/// `next_radio`) reads it at in that state and citing the line that names it
/// (widgets/checkbox.rs, `CheckboxLook::next_sized`), the fill at rest where
/// no rule states the hovered one. The mark is claimed where it is painted:
/// the widget fades it out while unchecked.
#[cfg(feature = "widgets")]
fn native_indicator(
    info: WidgetInfo,
    r: &NextTheme,
    radio: bool,
    look: &CheckboxLook,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let indicator = |state: WidgetState| {
        if radio {
            r.radio.indicator(state)
        } else {
            r.checkbox.indicator(state)
        }
    };
    // A disabled fill a rule states replaces the box and its mark's colour;
    // with none the box is its enabled self (CheckboxLook::next_of).
    let stated_disabled = disabled
        && indicator(WidgetState::DISABLED)
            .from_state
            .contains(PartStated::FILL);
    let info = info.color(match (radio, stated_disabled, checked) {
        (false, true, false) => claim(
            "fill",
            "checkbox.indicator(DISABLED).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:272",
        ),
        (false, true, true) => claim(
            "fill",
            "checkbox.indicator(DISABLED|CHECKED).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:273",
        ),
        (true, true, false) => claim(
            "fill",
            "radio.indicator(DISABLED).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:274",
        ),
        (true, true, true) => claim(
            "fill",
            "radio.indicator(DISABLED|CHECKED).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:275",
        ),
        (false, false, true) => claim(
            "fill",
            "checkbox.indicator(CHECKED).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:330",
        ),
        (true, false, true) => claim(
            "fill",
            "radio.indicator(CHECKED).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:331",
        ),
        (false, false, false) => claim(
            "fill",
            "checkbox.indicator(NONE).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:323",
        ),
        (true, false, false) => claim(
            "fill",
            "radio.indicator(NONE).fill",
            look.fill,
            "native-theme-gpui/widgets/checkbox.rs:324",
        ),
    });
    // An unset hover is the box at rest (CheckboxLook::next_of); the swatch
    // is the fill drawn under the pointer, the layer composited over the
    // fill.
    let hover_stated = state_fill_of(&indicator(WidgetState::HOVER)).is_some();
    let info = info.colors(match (radio, hover_stated) {
        (false, true) => stated_claim(
            "hover, checkbox.indicator(HOVER).fill over the fill",
            "checkbox.indicator(HOVER).fill",
            look.hover_fill,
            "native-theme-gpui/widgets/checkbox.rs:338",
        ),
        (true, true) => stated_claim(
            "hover, radio.indicator(HOVER).fill over the fill",
            "radio.indicator(HOVER).fill",
            look.hover_fill,
            "native-theme-gpui/widgets/checkbox.rs:339",
        ),
        (false, false) => stated_claim(
            "hover, none stated: the fill at rest",
            "checkbox.indicator(NONE).fill",
            look.hover_fill,
            "native-theme-gpui/widgets/checkbox.rs:323",
        ),
        (true, false) => stated_claim(
            "hover, none stated: the fill at rest",
            "radio.indicator(NONE).fill",
            look.hover_fill,
            "native-theme-gpui/widgets/checkbox.rs:324",
        ),
    });
    let width = Some(f32::from(look.border_width));
    let info = info.colors(match (radio, checked) {
        (false, false) => edge_claim(
            "hovered edge",
            "checkbox.indicator(HOVER).stroke.color",
            look.hover_border,
            width,
            "native-theme-gpui/widgets/checkbox.rs:288",
        ),
        (false, true) => edge_claim(
            "hovered edge",
            "checkbox.indicator(HOVER|CHECKED).stroke.color",
            look.hover_border,
            width,
            "native-theme-gpui/widgets/checkbox.rs:289",
        ),
        (true, false) => edge_claim(
            "hovered edge",
            "radio.indicator(HOVER).stroke.color",
            look.hover_border,
            width,
            "native-theme-gpui/widgets/checkbox.rs:290",
        ),
        (true, true) => edge_claim(
            "hovered edge",
            "radio.indicator(HOVER|CHECKED).stroke.color",
            look.hover_border,
            width,
            "native-theme-gpui/widgets/checkbox.rs:291",
        ),
    });
    let info = info.colors(match (radio, checked) {
        (false, true) => edge_claim(
            "border",
            "checkbox.indicator(CHECKED).stroke.color",
            Some(look.border),
            width,
            "native-theme-gpui/widgets/checkbox.rs:345",
        ),
        (true, true) => edge_claim(
            "border",
            "radio.indicator(CHECKED).stroke.color",
            Some(look.border),
            width,
            "native-theme-gpui/widgets/checkbox.rs:346",
        ),
        (false, false) => edge_claim(
            "border",
            "checkbox.indicator(NONE).stroke.color",
            Some(look.border),
            width,
            "native-theme-gpui/widgets/checkbox.rs:352",
        ),
        (true, false) => edge_claim(
            "border",
            "radio.indicator(NONE).stroke.color",
            Some(look.border),
            width,
            "native-theme-gpui/widgets/checkbox.rs:353",
        ),
    });
    // Over a stated disabled fill the mark takes the disabled label's colour.
    let info = info.colors(match (radio, stated_disabled) {
        (false, true) => stated_claim(
            "mark",
            "checkbox.label(DISABLED).color",
            checked.then_some(look.mark),
            "native-theme-gpui/widgets/checkbox.rs:365",
        ),
        (true, true) => stated_claim(
            "mark",
            "radio.label(DISABLED).color",
            checked.then_some(look.mark),
            "native-theme-gpui/widgets/checkbox.rs:366",
        ),
        (false, false) => stated_claim(
            "mark",
            "checkbox.checkmark(CHECKED).color",
            checked.then_some(look.mark),
            "native-theme-gpui/widgets/checkbox.rs:305",
        ),
        (true, false) => stated_claim(
            "mark",
            "radio.dot(CHECKED).fill",
            checked.then_some(look.mark),
            "native-theme-gpui/widgets/checkbox.rs:300",
        ),
    });
    info.color(match (radio, disabled) {
        (false, true) => claim(
            "label",
            "checkbox.label(DISABLED).color",
            look.label,
            "native-theme-gpui/widgets/checkbox.rs:365",
        ),
        (true, true) => claim(
            "label",
            "radio.label(DISABLED).color",
            look.label,
            "native-theme-gpui/widgets/checkbox.rs:366",
        ),
        (false, false) => claim(
            "label",
            "checkbox.label(NONE).color",
            look.label,
            "native-theme-gpui/widgets/checkbox.rs:360",
        ),
        (true, false) => claim(
            "label",
            "radio.label(NONE).color",
            look.label,
            "native-theme-gpui/widgets/checkbox.rs:360",
        ),
    })
}

/// A column of theme-drawn `widgets::Radio`s reading `labels`, the one at
/// `selected` selected (spec §2.2), under the installed 0.8.0 theme `r`:
/// what `CheckboxLook::next_radio` gives a selected radio and an unselected
/// one, the one source the widget draws from.
#[cfg(feature = "widgets")]
pub fn native_radio_column(
    r: &NextTheme,
    labels: &[&'static str],
    selected: Option<usize>,
) -> WidgetInfo {
    let chosen = selected
        .and_then(|ix| labels.get(ix).copied())
        .unwrap_or("none");
    let info = WidgetInfo::new("RadioGroup").variant("vertical");
    let radios = labels.join(", ");
    let drawn = drawn_or_upstream(
        CheckboxLook::next_radio(r, true, false)
            .and_then(|on| CheckboxLook::next_radio(r, false, false).map(|off| (on, off)))
            .map_err(|unstated| unstated.home()),
        next_ground(r),
        unstated_label_font(&r.radio.label(WidgetState::NONE), "radio.label(NONE)"),
    );
    let ((on, off), ground) = match drawn {
        Ok(drawn) => drawn,
        Err(home) => {
            return drawn_by_upstream(info, "Radio", home)
                .instance("radios", radios)
                .instance("selected", chosen);
        }
    };
    // The page's radios are never disabled, so `faded_over` leaves both
    // looks as they are; they go through it as the widget's do.
    let unselected = off.faded_over(ground);
    // The selected radio's colours, then the unselected ones'.
    let info = native_indicator(
        info.config("drawn by", DRAWN_BY),
        r,
        true,
        &on.faded_over(ground),
        true,
        false,
    );
    let info = info.color(claim(
        "unselected fill",
        "radio.indicator(NONE).fill",
        unselected.fill,
        "native-theme-gpui/widgets/checkbox.rs:324",
    ));
    let width = Some(f32::from(unselected.border_width));
    let info = info.colors(edge_claim(
        "unselected border",
        "radio.indicator(NONE).stroke.color",
        Some(unselected.border),
        width,
        "native-theme-gpui/widgets/checkbox.rs:353",
    ));
    let info = info.colors(
        if state_fill_of(&r.radio.indicator(WidgetState::HOVER)).is_some() {
            stated_claim(
                "hover, radio.indicator(HOVER).fill over an unselected fill",
                "radio.indicator(HOVER).fill",
                unselected.hover_fill,
                "native-theme-gpui/widgets/checkbox.rs:339",
            )
        } else {
            stated_claim(
                "hover, none stated: an unselected fill at rest",
                "radio.indicator(NONE).fill",
                unselected.hover_fill,
                "native-theme-gpui/widgets/checkbox.rs:324",
            )
        },
    );
    let info = info
        .config("indicator", format!("radio.indicator(NONE).min_size[0]: a circle {} across (platform-facts §2.5: radio buttons are circular)", length_text(on.indicator)))
        .config(
            "label gap",
            format!("radio.label_gap: {}", length_text(on.label_gap)),
        );
    let info = match (on.dot, on.mark_stroke) {
        (Some(dot), _) => info.config("dot", format!("radio.dot(NONE).min_size[0]: a dot {} across in radio.dot(CHECKED).fill, centred in the circle (platform-facts §2.5)", length_text(dot))),
        (None, Some(stroke)) => info.config("mark", format!(
            "the theme states no radio dot size (radio.dot(NONE).min_size[0]), so the connector strokes upstream's check polyline at checkbox.checkmark(NONE).stroke_width, {}, in a {} square inside the circle (native-theme-gpui/widgets/checkbox.rs, stroked_check)",
            length_text(stroke),
            length_text(on.mark_size)
        )),
        (None, None) => info.not_themeable("mark", super::own_icons("Check, gpui-component's check glyph, which widgets::Radio draws where the theme states no radio dot size (radio.dot(NONE).min_size[0]) and no checkbox.checkmark(NONE).stroke_width (native-theme-gpui/widgets/checkbox.rs, indicator_and_label)")),
    };
    let info = info.instance(
        "radios",
        format!("{radios}, each a widgets::Radio told its place in the group, widget_gap apart"),
    );
    // The focused radio's row draws the ring, as gpui-component's and the
    // egui connector's do.
    next_focus_ring(info.instance("selected", chosen), r, "row")
}

/// A theme-drawn switch's `info` where the theme states no value gpui can
/// take at `home`, and `widgets::Switch` draws gpui-component's own in its
/// place (native-theme-gpui/widgets/switch.rs, `Switch::row`): the switch
/// reading `label`, `checked` or not, `disabled` or not.
#[cfg(feature = "widgets")]
pub(crate) fn switch_by_upstream(
    info: WidgetInfo,
    home: &str,
    label: &'static str,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let info = info.config("drawn by", format!("gpui-component's own Switch, in upstream's colours and sizes: the theme states no value gpui can take at {home}, so widgets::Switch draws upstream's in its place (native-theme-gpui/widgets/switch.rs, Switch::row)"));
    toggled(info.instance("label", label), SWITCH, checked, disabled)
}

/// A theme-drawn `widgets::Switch` reading `label`, `checked` or not,
/// `disabled` or not (spec §2.3), under the installed 0.8.0 theme `r`: what
/// `SwitchLook::next_of` gives it, the one source the widget draws from,
/// faded as the widget fades a disabled one.
#[cfg(feature = "widgets")]
pub fn native_switch(
    r: &NextTheme,
    label: &'static str,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let info = WidgetInfo::new("Switch");
    let font = SwitchLabelFont::next_of(r);
    let drawn = drawn_or_upstream(
        SwitchLook::next_of(r, checked, disabled).map_err(|unstated| unstated.home()),
        next_ground(r),
        font.as_ref().err().map(|unstated| unstated.home()),
    );
    let (rest, ground) = match drawn {
        Ok(drawn) => drawn,
        Err(home) => return switch_by_upstream(info, home, label, checked, disabled),
    };
    let look = rest.faded_over(ground);
    let info = info.config("drawn by", DRAWN_BY);
    // Each claim names the home SwitchLook::next_of reads in this state and
    // cites the line that spells it (widgets/switch.rs,
    // SwitchLook::next_of); a state no rule states reads as the switch at
    // rest.
    let info = info.color(match (checked, disabled) {
        (false, false) => claim(
            "track",
            "switch.track(NONE).fill",
            look.track,
            "native-theme-gpui/widgets/switch.rs:263",
        ),
        (true, false) => claim(
            "track",
            "switch.track(CHECKED).fill",
            look.track,
            "native-theme-gpui/widgets/switch.rs:264",
        ),
        (false, true) => claim(
            "track",
            "switch.track(DISABLED).fill",
            look.track,
            "native-theme-gpui/widgets/switch.rs:265",
        ),
        (true, true) => claim(
            "track",
            "switch.track(DISABLED|CHECKED).fill",
            look.track,
            "native-theme-gpui/widgets/switch.rs:266",
        ),
    });
    // The swatch is the track drawn under the pointer, the layer composited
    // over it, the track at rest where no rule states one; a disabled
    // switch has none.
    let info = info.colors(match (checked, switch_hover_stated(r, checked)) {
        (false, true) => stated_claim(
            "hover, switch.track(HOVER).fill over the track",
            "switch.track(HOVER).fill",
            look.hover_track,
            "native-theme-gpui/widgets/switch.rs:247",
        ),
        (true, true) => stated_claim(
            "hover, switch.track(HOVER|CHECKED).fill over the track",
            "switch.track(HOVER|CHECKED).fill",
            look.hover_track,
            "native-theme-gpui/widgets/switch.rs:241",
        ),
        (false, false) => stated_claim(
            "hover, none stated: the track at rest",
            "switch.track(NONE).fill",
            look.hover_track,
            "native-theme-gpui/widgets/switch.rs:232",
        ),
        (true, false) => stated_claim(
            "hover, none stated: the track at rest",
            "switch.track(CHECKED).fill",
            look.hover_track,
            "native-theme-gpui/widgets/switch.rs:232",
        ),
    });
    // The track's outline and the thumb's, where the theme states the colour
    // and the width (SwitchLook::next_of).
    let width = Some(f32::from(look.track_border_width));
    let info = info.colors(match (checked, disabled) {
        (false, false) => edge_claim(
            "track outline",
            "switch.track(NONE).stroke.color",
            look.track_border,
            width,
            "native-theme-gpui/widgets/switch.rs:282",
        ),
        (true, false) => edge_claim(
            "track outline",
            "switch.track(CHECKED).stroke.color",
            look.track_border,
            width,
            "native-theme-gpui/widgets/switch.rs:283",
        ),
        (false, true) => edge_claim(
            "track outline",
            "switch.track(DISABLED).stroke.color",
            look.track_border,
            width,
            "native-theme-gpui/widgets/switch.rs:284",
        ),
        (true, true) => edge_claim(
            "track outline",
            "switch.track(DISABLED|CHECKED).stroke.color",
            look.track_border,
            width,
            "native-theme-gpui/widgets/switch.rs:285",
        ),
    });
    let width = Some(f32::from(look.thumb_border_width));
    let info = info.colors(if checked {
        edge_claim(
            "thumb outline",
            "switch.thumb(CHECKED).stroke.color",
            look.thumb_border,
            width,
            "native-theme-gpui/widgets/switch.rs:293",
        )
    } else {
        edge_claim(
            "thumb outline",
            "switch.thumb(NONE).stroke.color",
            look.thumb_border,
            width,
            "native-theme-gpui/widgets/switch.rs:292",
        )
    });
    // While focused the track draws the theme's focus ring; a disabled switch
    // takes no focus.
    let info = if disabled {
        info
    } else {
        next_focus_ring(info, r, "track")
    };
    let info = info.color(match (checked, disabled) {
        (false, false) => claim(
            "thumb",
            "switch.thumb(NONE).fill",
            look.thumb_color,
            "native-theme-gpui/widgets/switch.rs:273",
        ),
        (true, false) => claim(
            "thumb",
            "switch.thumb(CHECKED).fill",
            look.thumb_color,
            "native-theme-gpui/widgets/switch.rs:274",
        ),
        (false, true) => claim(
            "thumb",
            "switch.thumb(DISABLED).fill",
            look.thumb_color,
            "native-theme-gpui/widgets/switch.rs:275",
        ),
        (true, true) => claim(
            "thumb",
            "switch.thumb(DISABLED|CHECKED).fill",
            look.thumb_color,
            "native-theme-gpui/widgets/switch.rs:276",
        ),
    });
    // The label at the switch's own home where the theme states its colour,
    // else gpui-component's own, the window's text its switch inherits,
    // disabled muted_foreground (gpui-component/switch.rs, Switch::render),
    // at the homes the connector fills those tokens from.
    let info = info.color(match SwitchLook::next_label_home(r, checked, disabled) {
        Ok(SwitchLabelHome::Rest) => claim(
            "label",
            "switch.label(NONE).color",
            look.label,
            "native-theme-gpui/widgets/switch.rs:437",
        ),
        Ok(SwitchLabelHome::Checked) => claim(
            "label",
            "switch.label(CHECKED).color",
            look.label,
            "native-theme-gpui/widgets/switch.rs:438",
        ),
        Ok(SwitchLabelHome::Disabled) => claim(
            "label",
            "switch.label(DISABLED).color",
            look.label,
            "native-theme-gpui/widgets/switch.rs:439",
        ),
        Ok(SwitchLabelHome::DisabledChecked) => claim(
            "label",
            "switch.label(DISABLED|CHECKED).color",
            look.label,
            "native-theme-gpui/widgets/switch.rs:440",
        ),
        Ok(SwitchLabelHome::WindowForegroundMuted) => claim(
            "label",
            "colors.window.foreground_muted",
            look.label,
            "native-theme-gpui/widgets/switch.rs:488",
        ),
        _ => claim(
            "label",
            "colors.window.foreground",
            look.label,
            "native-theme-gpui/widgets/switch.rs:484",
        ),
    });
    let thumb_home = if checked {
        "switch.thumb(CHECKED).min_size[0]"
    } else {
        "switch.thumb(NONE).min_size[0]"
    };
    // An outline is drawn where the theme states its colour and its width.
    let outline = |edge: Option<Hsla>, width: Pixels, home: &str| match edge
        .filter(|_| width > gpui::px(0.))
    {
        Some(_) => format!("{home}, {}", length_text(width)),
        None => "none".to_owned(),
    };
    let info = info
        .config(
            "track",
            format!(
                "switch.track(NONE).min_size: {} × {}, corners switch.track(NONE).corners {}",
                length_text(look.track_width),
                length_text(look.track_height),
                length_text(look.track_radius)
            ),
        )
        .config(
            "thumb",
            format!(
                "{thumb_home}: {}, centred on the track's axis",
                length_text(look.thumb)
            ),
        )
        .config(
            "outlines",
            format!(
                "track: {}; thumb: {}; each inside its part, drawn where the theme states its colour and its width (native-theme-gpui/widgets/switch.rs, SwitchLook::next_of)",
                outline(look.track_border, look.track_border_width, "switch.track(NONE).stroke.width"),
                outline(look.thumb_border, look.thumb_border_width, "switch.thumb(NONE).stroke.width"),
            ),
        )
        .config("label font", switch_label_font_text(font.ok().as_ref()))
        .config(
            "label gap",
            match look.label_gap {
                Some(gap) => format!(
                    "switch.label_gap: {} (native-theme-gpui/widgets/switch.rs, SwitchLook::next_of)",
                    length_text(gap)
                ),
                None => "none stated at switch.label_gap: gpui-component's own gap_2 (gpui-component/switch.rs, Switch::render)".to_owned(),
            },
        );
    let info = if disabled {
        info.config("disabled", if rest.opacity < 1. {
            format!("switch.disabled_opacity {}: the whole control faded as one over the window's colors.window.background, so each colour above is composited over what it is drawn on and faded by it (native-theme-gpui/widgets/switch.rs, SwitchLook::faded_over)", percent_text(rest.opacity))
        } else {
            format!("switch.disabled_opacity {}: no fade, the colours above as the theme states them (native-theme-gpui/widgets/switch.rs, SwitchLook::faded_over)", percent_text(rest.opacity))
        })
    } else {
        info
    };
    toggled(info.instance("label", label), SWITCH, checked, disabled)
}

/// A theme-drawn slider's `info` where the theme states no value gpui can
/// take at `home`, and `widgets::Slider` draws gpui-component's own in its
/// place (native-theme-gpui/widgets/slider.rs, `Slider::render`), with the
/// keyboard line of a `range` one or a single thumb.
#[cfg(feature = "widgets")]
pub(crate) fn slider_by_upstream(info: WidgetInfo, home: &str, range: bool) -> WidgetInfo {
    let info = info.config("drawn by", format!("gpui-component's own Slider, in upstream's colours and sizes: the theme states no value gpui can take at {home}, so widgets::Slider draws upstream's in its place (native-theme-gpui/widgets/slider.rs, Slider::render)"));
    slider_keys(info, range)
}

/// A theme-drawn `widgets::Slider` (spec §2.4) under the installed 0.8.0
/// theme `r`: what `SliderLook::next_of` and `ThumbOutline::next_of` give
/// it, the one source the widget draws from.
#[cfg(feature = "widgets")]
pub fn native_slider(r: &NextTheme, vertical: bool, range: bool) -> WidgetInfo {
    let info = slider_shape(WidgetInfo::new("Slider"), vertical, range);
    let drawn = drawn_or_upstream(
        SliderLook::next_of(r, false).map_err(|unstated| unstated.home()),
        next_ground(r),
        None,
    );
    let (rest, ground) = match drawn {
        Ok(drawn) => drawn,
        Err(home) => return slider_by_upstream(info, home, range),
    };
    let look = rest.faded_over(ground);
    // Each claim names the home SliderLook::next_of reads and cites the line
    // that spells it (widgets/slider.rs, SliderLook::next_of). The hover
    // swatch is the thumb drawn under the pointer, the layer composited over
    // it, the thumb at rest where no rule states one.
    let info = info
        .config("drawn by", DRAWN_BY)
        .color(claim(
            "rail",
            "slider.track(NONE).fill",
            look.track,
            "native-theme-gpui/widgets/slider.rs:293",
        ))
        .color(claim(
            "filled part",
            "slider.active_track(NONE).fill",
            look.fill,
            "native-theme-gpui/widgets/slider.rs:298",
        ))
        .color(claim(
            "thumb",
            "slider.thumb(NONE).fill",
            look.thumb_color,
            "native-theme-gpui/widgets/slider.rs:304",
        ));
    let info = info.colors(
        if state_fill_of(&r.slider.thumb(WidgetState::HOVER)).is_some() {
            stated_claim(
                "hover, slider.thumb(HOVER).fill over the thumb",
                "slider.thumb(HOVER).fill",
                look.hover_thumb,
                "native-theme-gpui/widgets/slider.rs:307",
            )
        } else {
            stated_claim(
                "hover, none stated: the thumb at rest",
                "slider.thumb(NONE).fill",
                look.hover_thumb,
                "native-theme-gpui/widgets/slider.rs:306",
            )
        },
    );
    let info = info
        .config(
            "rail",
            format!(
                "slider.track(NONE).min_size[1]: {}",
                length_text(look.track_height)
            ),
        )
        .config(
            "thumb",
            format!(
                "slider.thumb(NONE).min_size[0]: {}; its centre travels the width less a radius at each end",
                length_text(look.thumb)
            ),
        )
        .not_themeable("rail radius", "a pill, gpui-component's radius_full (gpui-component/slider.rs, Slider::render); the model states no rail radius the widget reads");
    let info = native_thumb_outline(info, r, ground, &rest);
    let info = if vertical {
        info.not_themeable("length", "the caller's height; gpui-component's 120px where none is given (gpui-component/slider.rs, Slider::render): the model states no slider length")
    } else {
        info
    };
    // The focused end thumb draws the ring.
    slider_keys(next_focus_ring(info, r, "end thumb"), range)
}

/// A theme-drawn slider's thumb outline in `info`, as the widget draws it
/// (widgets/slider.rs, `Slider::render`): `ThumbOutline::next_of` of `r`
/// faded over `ground` with `rest`, the slider's look before its own fade,
/// each colour claimed at the home the outline reads it at, its hovered and
/// focused colours where a rule states them; where the theme states no
/// stroke gpui can draw there, gpui-component's own ring.
#[cfg(feature = "widgets")]
fn native_thumb_outline(
    info: WidgetInfo,
    r: &NextTheme,
    ground: Hsla,
    rest: &SliderLook,
) -> WidgetInfo {
    let read = ThumbOutline::next_of(r, false);
    let Some(outline) = read.ok().flatten().map(|o| o.faded_over(ground, rest)) else {
        return upstream_thumb_outline(info, read.err().map(|unstated| unstated.home()));
    };
    if outline.width <= gpui::px(0.) {
        return info.config("thumb outline", "none: slider.thumb(NONE).stroke is none or 0 wide, so the thumb draws no outline in any state, gpui-component's ring included");
    }
    let width = Some(f32::from(outline.width));
    let ruled = |state| state_stroke_color_of(&r.slider.thumb(state)).is_some();
    info.colors(edge_claim("thumb outline", "slider.thumb(NONE).stroke.color", outline.rest, width, "native-theme-gpui/widgets/slider.rs:201"))
        .colors(edge_claim("thumb outline, hovered", "slider.thumb(HOVER).stroke.color", outline.hover.filter(|_| ruled(WidgetState::HOVER)), width, "native-theme-gpui/widgets/slider.rs:211"))
        .colors(edge_claim("thumb outline, focused", "slider.thumb(FOCUSED).stroke.color", outline.focus.filter(|_| ruled(WidgetState::FOCUSED)), width, "native-theme-gpui/widgets/slider.rs:215"))
        .config("thumb outline", format!("slider.thumb(NONE).stroke.width, {}, a band inside the thumb (widgets::ThumbOutline): slider.thumb(NONE).stroke.color at rest; slider.thumb(HOVER).stroke.color with the pointer on the thumb, though not while it is dragged, gpui applying no hover style during a drag (gpui-pre/elements/div.rs, Interactivity::compute_style_internal); slider.thumb(FOCUSED).stroke.color on the end thumb while the slider has the focus, from a key or a press on the rail (a press on a thumb gives none: gpui-base/slider.rs, SliderThumb), the pointer's winning; each a rule does not state the rest colour (widgets/slider.rs, thumb)", length_text(outline.width)))
}

/// A theme-drawn slider's thumb outline in `info` where it is
/// gpui-component's own ring: the theme states no stroke gpui can draw at
/// `unstated`, or, where that is `None`, no `slider.thumb(NONE).stroke`.
#[cfg(feature = "widgets")]
pub(crate) fn upstream_thumb_outline(info: WidgetInfo, unstated: Option<&str>) -> WidgetInfo {
    let upstream_ring = "gpui-component's own: a 1px ring of the fill colour at half alpha (gpui-component/slider.rs, Slider::render)";
    info.not_themeable(
        "thumb outline",
        match unstated {
            Some(home) => {
                format!("{upstream_ring}; the theme states no stroke gpui can draw at {home}")
            }
            None => format!("{upstream_ring}; the theme states no slider.thumb(NONE).stroke"),
        },
    )
}

/// A theme-drawn slider's `info` with its keyboard line, a `range` one's
/// or a single thumb's.
#[cfg(feature = "widgets")]
fn slider_keys(info: WidgetInfo, range: bool) -> WidgetInfo {
    if range {
        info.instance("keyboard", "Left/Down and Right/Up step the range's end, Home takes it down to the start, End to the maximum; the start stays")
    } else {
        info.instance(
            "keyboard",
            "Left/Down and Right/Up step it, Home and End go to the ends",
        )
    }
}

pub fn rating(t: &Theme, value: usize, disabled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Rating").not_themeable(
        "own icons",
        super::own_icons("Star, and StarFill for a star at or below the value (rating.rs, Rating)"),
    );
    let info = if disabled {
        info.variant("disabled")
    } else {
        info
    };
    let info = info
        .color(claim(
            "active star",
            "yellow",
            t.yellow,
            "gpui-component/rating.rs:121",
        ))
        // Rating colours only a filled or hovered star (rating.rs:159), so
        // an empty one takes the colour the showcase sets on its window.
        .color(claim(
            "inactive star, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .not_themeable("active colour", "cx.theme().yellow unless Rating::color overrides it (rating.rs, Rating::render active_color)")
        .not_themeable("inactive colour", "none of its own: Rating colours only a filled or hovered star, so an empty one takes the window's text colour (rating.rs, Rating::render)");
    let info = if disabled {
        info.not_themeable("hover preview", "none: a disabled Rating gives its stars no pointer handlers, so it neither previews a value nor takes a click (rating.rs, Rating::render)")
    } else {
        info.not_themeable(
            "hover preview",
            "upstream keeps its own hovered value (rating.rs, RaitingState::hovered_value)",
        )
    };
    let info = info.instance("value", format!("{value} of 5 stars"));
    if disabled {
        info
    } else {
        info.instance("click", "a star above the value sets it there, and one at or below it clears down to the star before (rating.rs, Rating::render); the showcase keeps the value")
    }
}

/// An `OtpInput`.
pub fn otp_input(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("OtpInput")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/otp_input.rs:109",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/input/otp_input.rs:113",
        ))
        .color(input_background(t))
        .color(claim(
            "digit",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:127",
        ))
        .color(claim(
            "caret",
            "caret",
            t.caret,
            "gpui-component/input/otp_input.rs:153",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("fill", "input_background(), as an Input's, and the digits take the Input's foreground. The secondary_foreground and muted_foreground the panel had named colour a masked asterisk, and this OtpState is not masked (input/otp_input.rs, OtpInput)")
        .instance("digit count", "configurable")
        .instance("groups", "2")
}

/// A `Select`. Its geometry line is recorded where `demo::select` applies
/// the builder.
pub fn select(t: &Theme, native: Option<&NextTheme>) -> WidgetInfo {
    let info = combo_fill(WidgetInfo::new("Select"), t, native, false);
    // The trigger sets text_color(foreground) and then refines its style
    // (gpui-component/select.rs, SelectState::render), so geometry::select's
    // combo_box.label(NONE) colour wins where it applies.
    let refined = native.filter(|r| native_theme_gpui::next::geometry::select(r).is_ok());
    let info = match refined {
        Some(_) => info.instance("trigger text", "combo_box.label(NONE).color, which geometry::select letters the trigger in over upstream's foreground: the trigger sets its text colour and then refines its style (gpui-component/select.rs, SelectState::render)"),
        None => info.color(claim(
            "upstream trigger text",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:127",
        )),
    };
    info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/select.rs:597",
        ))
        .color(claim(
            "placeholder",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/select.rs:492",
        ))
        .not_themeable("carried colour", "the one difference from a Combobox, such as the theme settings' preset switch: Select's selected-title child sets its own colour, so a carried colour yields to the disabled colour instead of beating it, and the connector carries it (native-theme-gpui/geometry.rs, combobox)")
        .not_themeable("caret", "its colour is themed -- upstream paints it with muted_foreground (select.rs, Caret) -- and its size is not: Caret maps Size::Size into the same arm as Medium (gpui-component/select.rs, Caret::render), so combo_box.arrow_icon_size has no route at all, not even through the Size::Size escape hatch a DataTable row accepts. Tier U for the size")
        .not_themeable("own icons", super::own_icons("the caret's ChevronDown (select.rs, Caret), and in its open list Check beside the chosen row (searchable_list/item.rs, SearchableListItemElement) and Inbox while the list is empty (gpui-component/select.rs, SelectState::new)"))
}

/// `info` of a `Select` built searchable, with `highlight_matches(true)`:
/// the colour its rows draw a match in, where it comes from, and, where
/// `native` (a native theme is installed), the limit of that colour.
pub fn match_highlight(info: WidgetInfo, t: &Theme, native: bool) -> WidgetInfo {
    let info = info
        .variant("searchable, highlight_matches(true)")
        .color(claim(
            "match",
            "list_highlight_foreground",
            t.list_highlight_foreground,
            "gpui-component/label.rs:266",
        ))
        .instance("highlight", "typed into its search field, the query filters the rows, and each row's text that matches it, case-insensitively, is drawn in list_highlight_foreground; the rest of the row keeps the row's own colour (gpui-component/label.rs, highlight_matches; searchable_list/delegate.rs, render_highlighted)")
        .not_themeable("search field", "at the top of its open list, with a rule in border under it: gpui-component's own Input without its frame, led by a Search icon in muted_foreground, gpui-component's own whichever icon theme is chosen, and cleanable (gpui-component/list/list.rs:720-740, query_input)");
    if native {
        info.config("match colour", "list_highlight_foreground, which the connector fills with the window's text colour, colors.window.foreground: the model states no colour for a match (native-theme-gpui/next/mod.rs, slice_colors; native-theme-gpui/colors.rs, assign_list_table_slice). So the match reads as the rest of the row at rest only: a hovered row and the selected row alike are lettered accent_foreground, which the connector fills with menu.item_label(HOVER).color (native-theme-gpui/next/mod.rs, widget_colors; native-theme-gpui/colors.rs, assign_core; gpui-component/searchable_list/item.rs, SearchableListItemElement), and on both the match stands out in the resting colour")
            .not_themeable("match limit", "no one colour reads in all three row states on every preset: this one never falls below a contrast of 1.5 at rest or hovered, and does on the selected row in 7 of the 34 bundled preset and mode combinations, against 20 for the info colour, 10 of them invisible. Upstream's highlight replaces the row's text colour with the one colour; SearchableListItem::render_highlighted is the way to draw a match that keeps the row's text colour")
    } else {
        info.not_themeable("match colour", "list_highlight_foreground, gpui-component's own: no native theme is installed, so its built-in theme's, blue-600 in light mode and blue-400 in dark (gpui-component/theme/default-theme.json), where a theme that states none falls back to its blue (gpui-component/theme/schema.rs, apply_config)")
    }
}

/// A drop-down trigger's fill and edge: under a 0.8.0 theme `native` that
/// states the trigger's look (`demo::combo_look`),
/// `combo_box.field(NONE).fill`, `geometry::combo_box_fill`'s, which the
/// showcase paints under the trigger (`demo::combo_surface`), and
/// `combo_box.field(NONE).stroke.color`, which the showcase refines the
/// trigger with over upstream's (`demo::combo_fill`); upstream's
/// `input_background()` and `input` otherwise, the edge read by a
/// `Combobox` where `combobox`, by a `Select` where not.
pub fn combo_fill(
    info: WidgetInfo,
    t: &Theme,
    native: Option<&NextTheme>,
    combobox: bool,
) -> WidgetInfo {
    let look = native.map(|r| (r, crate::demo::combo_look(r)));
    match look {
        Some((r, Ok(_))) => {
            let width = stroke_width_of(&r.combo_box.field(WidgetState::NONE));
            let info = info
                .colors(stated_claim(
                    "trigger fill",
                    "combo_box.field(NONE).fill",
                    fill_of(&r.combo_box.field(WidgetState::NONE)),
                    "showcase",
                ))
                .colors(edge_claim(
                    "trigger border",
                    "combo_box.field(NONE).stroke.color",
                    stroke_color_of(&r.combo_box.field(WidgetState::NONE)),
                    width,
                    "showcase",
                ))
                .colors(stated_claim(
                    "hover layer, over the fill",
                    "combo_box.field(HOVER).fill",
                    state_fill_of(&r.combo_box.field(WidgetState::HOVER)),
                    "showcase",
                ))
                .colors(edge_claim(
                    "hovered edge, in place of the trigger's",
                    "combo_box.field(HOVER).stroke.color",
                    state_stroke_color_of(&r.combo_box.field(WidgetState::HOVER)),
                    width,
                    "showcase",
                ))
                .next_geometry("combo_box_fill");
            info.config("fill", "geometry::combo_box_fill's combo_box.field(NONE).fill, and combo_box.field(HOVER).fill over it under the pointer where a rule states it, which the showcase paints on the element under the trigger and leaves the trigger without a fill: upstream's trigger sets no hover and is not an InteractiveElement (gpui-component/select.rs, Select::render; gpui-component/combobox.rs, Combobox::render); the trigger's edge is combo_box.field(NONE)'s stroke, none where it is 0 wide, over the input edge it paints first, which the showcase sets: geometry::select and geometry::combobox carry no edge colour, since the trigger of a Select or Combobox built with appearance(false) keeps a transparent edge the refinement would colour (gpui-component/select.rs, Select::render; gpui-component/combobox.rs, Combobox::render); and, where a rule states it, combo_box.field(HOVER).stroke.color in place of that edge under the pointer, none where the rule states no stroke, for the same reason: the showcase then draws the edge on an element under the trigger, combo_box.field(NONE)'s at rest, and leaves the trigger's own transparent, so the focused trigger's ring edge stays over it")
        }
        Some((_, Err(home))) => unstated_combo_fill(info, t, combobox, home),
        None => upstream_combo_fill(info, t, combobox),
    }
}

/// [`combo_fill`] where an installed 0.8.0 theme leaves `home` without a
/// value gpui can take, which the trigger's surface needs
/// (`demo::combo_look`): the trigger keeps upstream's fill and edge, and
/// says at which home.
pub(crate) fn unstated_combo_fill(
    info: WidgetInfo,
    t: &Theme,
    combobox: bool,
    home: &str,
) -> WidgetInfo {
    upstream_combo_fill(info, t, combobox).config(
        "surface",
        format!("none: the theme states no value gpui can take at {home}, which the trigger's fill and edge the showcase draws need; the trigger keeps upstream's"),
    )
}

/// [`combo_fill`] where the trigger keeps upstream's fill and edge.
fn upstream_combo_fill(info: WidgetInfo, t: &Theme, combobox: bool) -> WidgetInfo {
    info.color(input_background(t))
            .color(if combobox {
                claim(
                    "trigger border",
                    "input",
                    t.input,
                    "gpui-component/combobox.rs:1037",
                )
            } else {
                claim(
                    "trigger border",
                    "input",
                    t.input,
                    "gpui-component/select.rs:590",
                )
            })
            .not_themeable("fill", "input_background(), as an Input's: the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs, input_background)")
}

/// A `ColorPicker`.
pub fn color_picker(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("ColorPicker")
        .color(claim(
            "popover",
            "popover",
            t.popover,
            "gpui-component/styled.rs:190",
        ))
        .color(claim(
            "featured red",
            "red",
            t.red,
            "gpui-component/color_picker.rs:212",
        ))
        .color(claim(
            "featured red light",
            "red_light",
            t.red_light,
            "gpui-component/color_picker.rs:213",
        ))
        .color(claim(
            "featured blue",
            "blue",
            t.blue,
            "gpui-component/color_picker.rs:214",
        ))
        .color(claim(
            "featured blue light",
            "blue_light",
            t.blue_light,
            "gpui-component/color_picker.rs:215",
        ))
        .color(claim(
            "featured green",
            "green",
            t.green,
            "gpui-component/color_picker.rs:216",
        ))
        .color(claim(
            "featured green light",
            "green_light",
            t.green_light,
            "gpui-component/color_picker.rs:217",
        ))
        .color(claim(
            "featured yellow",
            "yellow",
            t.yellow,
            "gpui-component/color_picker.rs:218",
        ))
        .color(claim(
            "featured yellow light",
            "yellow_light",
            t.yellow_light,
            "gpui-component/color_picker.rs:219",
        ))
        .color(claim(
            "featured cyan",
            "cyan",
            t.cyan,
            "gpui-component/color_picker.rs:220",
        ))
        .color(claim(
            "featured cyan light",
            "cyan_light",
            t.cyan_light,
            "gpui-component/color_picker.rs:221",
        ))
        .color(claim(
            "featured magenta",
            "magenta",
            t.magenta,
            "gpui-component/color_picker.rs:222",
        ))
        .color(claim(
            "featured magenta light",
            "magenta_light",
            t.magenta_light,
            "gpui-component/color_picker.rs:223",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("swatch", "the picked colour itself, edged with it darkened by 30%: background and input frame only an empty swatch, and this one starts with a value (color_picker.rs, ColorPickerButton)")
        .not_themeable("palette grid", "the rows under the featured swatches are gpui-component's own colour scales, not the platform's (color_picker.rs, color_palettes)")
}

/// A `ColorSelect`, empty until a colour is `picked`.
pub fn color_select(t: &Theme, picked: bool) -> WidgetInfo {
    let info = WidgetInfo::new("ColorSelect")
        .color(input_background(t))
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/color_picker.rs:773",
        ))
        .color(claim(
            "caret",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/color_picker.rs:812",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/color_picker.rs:778",
        ));
    let info = if t.focus_ring {
        info.color(claim(
            "focus ring, ring at 50% alpha",
            "ring",
            t.ring.alpha(0.5),
            "gpui-component/styled.rs:261",
        ))
    } else {
        info.config("focus ring", NO_BAND)
    };
    let info = if picked {
        info.color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:127",
        ))
        .instance("swatch", "the picked colour, edged in it darkened by 30% (color_picker.rs, ColorPickerButton::render_field)")
    } else {
        info.color(claim(
            "placeholder",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/color_picker.rs:806",
        ))
        .color(claim(
            "swatch edge",
            "input",
            t.input,
            "gpui-component/color_picker.rs:786",
        ))
    };
    info.not_themeable("frame", "the field is an inner element that sets its own height, padding and text size from the Input ladder (color_picker.rs, ColorPickerButton::render_field); a refinement lands on the outer wrapper and reaches only its width, so no geometry builder is applied")
        .not_themeable("own icons", super::own_icons("the caret's ChevronDown (select.rs, Caret)"))
}

/// A `SpeechWaveform` of `kind` over the showcase's synthetic input.
pub fn speech_waveform(t: &Theme, kind: WaveformKind) -> WidgetInfo {
    let info = WidgetInfo::new("SpeechWaveform")
        .instance("signal", "the showcase's own synthetic level envelope (speech.rs), no microphone; nothing is recognised");
    match kind {
        WaveformKind::Bars => info
            .variant("bars")
            .color(claim(
                "bars, capturing",
                "primary",
                t.primary,
                "gpui-component/speech/waveform.rs:137",
            ))
            .color(claim(
                "bars, idle",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/speech/waveform.rs:139",
            ))
            .not_themeable("size", "96px wide and 12 / 16 / 20 / 24px tall per Size, literals (speech/waveform.rs, DEFAULT_WIDTH and SpeechWaveform::height)")
            .not_themeable("motion", "one bar per 80ms of audio, scrolling while capturing; the bars stand still under reduced motion (speech/level.rs, LEVEL_INTERVAL; speech/waveform.rs, SpeechWaveform::render)"),
        WaveformKind::Liquid => info
            .variant("liquid")
            .color(claim(
                "surface, capturing",
                "primary",
                t.primary,
                "gpui-component/speech/waveform.rs:137",
            ))
            .color(claim(
                "surface, idle",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/speech/waveform.rs:139",
            ))
            .color(claim(
                "water at the crest, capturing, at 16%",
                "primary",
                t.primary.opacity(0.16),
                "gpui-component/speech/waveform.rs:137",
            ))
            .color(claim(
                "water at the crest, idle, at 16%",
                "muted_foreground",
                t.muted_foreground.opacity(0.16),
                "gpui-component/speech/waveform.rs:139",
            ))
            .not_themeable("size", "20px tall at the default Size, a literal, and as wide as its row, the showcase's w_full in place of upstream's 96px (gpui-component/speech/waveform.rs, SpeechWaveform::height and DEFAULT_WIDTH)")
            .not_themeable("surface", "a ribbon a tenth of the height thick in the middle, at least 2px, thinning to a quarter of that at its ends; crests reach 0.42 of the height, flattened by tanh at 1.4; the water fades from 16% at the crest to clear at the bottom (gpui-component/speech/waveform.rs:192-204, liquid; :26, CREST_RATIO; :30, CREST_SOFTNESS; :33, LINE_TAPER; :41, WATER_OPACITY): literals")
            .not_themeable("motion", "while capturing the surface is advanced every frame and the window redraws; under reduced motion it holds a ripple that grows with the level (gpui-component/speech/waveform.rs:132-135, :199-201, :209-217)"),
    }
}

/// The square of the speech indicator's stop Button, where the installed
/// 0.8.0 theme states it: the resting button body's minimum height, the
/// side, and its corner radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct StopSquare {
    /// `button.body(NONE).min_size[1]`.
    pub(crate) side: f32,
    /// `button.body(NONE).corners`, as one radius.
    pub(crate) radius: f32,
}

impl StopSquare {
    /// `r`'s square, `None` where it states no minimum height or no one
    /// radius for the resting button body.
    pub(crate) fn of(r: &NextTheme) -> Option<Self> {
        let body = r.button.body(WidgetState::NONE);
        Some(Self {
            side: body.min_size[1]?,
            radius: super::radius_of(&body)?,
        })
    }
}

/// The `SpeechIndicator` the Inputs page shows while its synthetic input
/// captures: the only child of a selected Ghost Button that stops the
/// session, a square of `native`'s side where the installed 0.8.0 theme
/// states one. `keyboard` is whether the Button has keyboard focus, under
/// which it shows its name in place of the indicator.
pub fn speech_indicator(t: &Theme, native: Option<StopSquare>, keyboard: bool) -> WidgetInfo {
    let info = super::buttons::native_ghost_selected(
        WidgetInfo::new("SpeechIndicator").variant(if keyboard {
            "in a stop Button, keyboard focus"
        } else {
            "in a stop Button"
        }),
        t,
    )
    .instance("signal", "the showcase's own synthetic level envelope (speech.rs), no microphone; the Button is shown only while that input captures, which is what the indicator is for (speech/indicator.rs:45, SpeechState)")
    .instance("click", "stops the session")
    .instance("colour", "the Button's text colour, the text claim: the indicator paints in the text colour it is drawn in (speech/indicator.rs:109-110, text_style), and so does its stop glyph (icon.rs:260-265, text_style)");
    let info = if keyboard {
        info.instance("keyboard focus", "the Button shows its name, Stop, in place of the indicator: keyboard focus has no pointer to reveal the stop glyph, so the indicator's doc asks for the glyph instead (speech/indicator.rs:55-57, SpeechButton), and the chosen icon theme has no stop icon, while another theme's is never shown")
    } else {
        info
    };
    let padding = if keyboard {
        "with upstream's side padding round the name, px_2p5 at the default Size (button/button.rs:700-703, px_2p5), so a longer name widens it"
    } else {
        "with no padding, so the indicator fills the Button as its doc asks (speech/indicator.rs:52-54, SpeechButton)"
    };
    let square = match native {
        Some(s) => info.config(
            "square",
            format!(
                "{}px, button.body(NONE).min_size[1], rounded by button.body(NONE).corners, {}px, {padding}; the showcase sets them on the Button (demo::speech_indicator)",
                px_text(s.side),
                px_text(s.radius.max(0.))
            ),
        ),
        None => info.not_themeable(
            "square",
            format!(
                "size_8, the square upstream gives an icon Button at the default Size, {padding}; a Button with a child is laid out as a labelled one (button/button.rs:693-699, size_8), so the showcase sets it, where the installed 0.8.0 theme states no value gpui can take at button.body(NONE).min_size[1] or button.body(NONE).corners"
            ),
        ),
    };
    square
        .not_themeable("bars", "four, each 0.085 of the indicator's shorter side wide and 0.075 apart, a full one 0.46 of it tall (speech/indicator.rs:16, BARS; :41-43, HEIGHT_RATIO), side by side round the centre and grown from the midline, never shorter than wide (speech/indicator.rs, bar_rects): literals")
        .not_themeable("motion", "each bar follows the level read at a playhead gliding between levels 80ms apart (speech/level.rs, LEVEL_INTERVAL; speech/indicator.rs:143-145, playhead), weighted 0.65 / 1 / 0.85 / 0.55 and lagging 1 / 0 / 0.5 / 1.5 levels (speech/indicator.rs:20, WEIGHTS; :24, LAGS); in silence they breathe up to 0.12 of the way from a dot to a full bar, at 2.4 radians per second and 0.9 radians apart (speech/indicator.rs:31, BREATH; :34, BREATH_RATE; :37, BREATH_SPREAD); before any audio they are dots (speech/indicator.rs:139-142, bar_heights); under reduced motion they hold still at 0.4 / 0.7 / 0.55 / 0.35 (speech/indicator.rs:27, STILL; :85-86, reduce_motion); the window redraws every frame while capturing, unless motion is reduced (speech/indicator.rs:92-94, request_animation_frame): literals")
        .not_themeable("hover", "under the pointer the bars fade out and the stop glyph, gpui-component's Square, fades in at the Size the indicator is given (speech/indicator.rs:104, group_hover; :128-130, Square): geometry::icon_size_toolbar where the installed 0.8.0 theme states toolbar.button_icon(NONE).size, the default Size otherwise")
        .not_themeable("own icons", super::own_icons("the stop glyph's Square (speech/indicator.rs, SpeechIndicator::render)"))
}

/// A `DatePicker`.
pub fn date_picker(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("DatePicker")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/time/date_picker.rs:729",
        ))
        .color(claim(
            "popover",
            "popover",
            t.popover,
            "gpui-component/styled.rs:190",
        ))
        .color(claim(
            "selected day",
            "primary",
            t.primary,
            "gpui-component/time/calendar.rs:190",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("calendar icon", "an IconName::Calendar built inline with no setter (time/date_picker.rs, DatePicker)")
        .not_themeable("own icons", super::own_icons("the trigger's Calendar (time/date_picker.rs, DatePicker), and ChevronLeft and ChevronRight on the month buttons of the Calendar it opens (time/calendar.rs, Calendar)"))
        .not_themeable("format", "%Y/%m/%d unless the application sets another, whatever the locale (time/date_picker.rs, DatePickerState::date_format)")
}

/// A `Calendar`.
pub fn calendar(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Calendar")
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/time/calendar.rs:200",
        ))
        .color(claim(
            "selected day",
            "primary",
            t.primary,
            "gpui-component/time/calendar.rs:190",
        ))
        .color(claim(
            "today",
            "accent",
            t.accent,
            "gpui-component/time/calendar.rs:194",
        ))
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/time/calendar.rs:185",
        ))
        .config("border-radius", format!("radius_lg: {}px", t.radius_lg.as_f32()))
        .not_themeable("fill", "none: a Calendar sets an edge, a radius and a padding but no background, so the window shows through (time/calendar.rs, Calendar)")
        .not_themeable("month navigation", "ChevronLeft and ChevronRight built inline with no setter (time/calendar.rs, Calendar)")
        .not_themeable("own icons", super::own_icons("ChevronLeft and ChevronRight on its month buttons (time/calendar.rs, Calendar)"))
}

/// The Inputs page's `TimeField` (spec §8.3), enabled and empty: a field of
/// hour, minute and period segments over the state `Showcase::new` builds.
pub fn time_field(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("TimeField")
        .color(super::chrome::input_background(t))
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:127",
        ))
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/time/time_field.rs:110",
        ))
        .color(claim(
            "border, invalid",
            "danger",
            t.danger,
            "gpui-component/time/time_field.rs:118",
        ))
        .color(claim(
            "selected segment",
            "selection",
            t.selection,
            "gpui-component/time/time_field.rs:137",
        ));
    let info = if t.focus_ring {
        info.color(claim(
            "focus ring, ring at 50% alpha",
            "ring",
            t.ring.alpha(0.5),
            "gpui-component/styled.rs:261",
        ))
    } else {
        // With the band off the bordered field (time/time_field.rs:109),
        // focused through focus_ring_style (:116), has its border tinted
        // (styled.rs:269-270).
        info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/styled.rs:270",
        ))
        .config("focus ring", NO_BAND)
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("segment radius", "half the theme's radius (time/time_field.rs, TimeField)")
        .not_themeable("size", "input_h and the input text size of its Size, Medium unless the caller sizes it -- rems, so the platform's font (time/time_field.rs, TimeField; sizing.rs, input_h)")
        .not_themeable("figures", "tabular figures, a font feature the field sets itself (time/time_field.rs, tabular_figures)")
        .instance("value", "none until one is typed: TimeFieldState::new holds no time (gpui-base/time_field.rs, TimeFieldState::new)")
}

/// `info`, an `Input`'s, for one that holds an inline token (spec §8.3),
/// drawn as upstream's `InputToken` chip; the token is not selected.
pub fn with_token(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.variant("with an inline token")
        .color(claim(
            "token bg",
            "muted",
            t.muted,
            "gpui-component/input/token.rs:53",
        ))
        .color(claim(
            "token border",
            "border",
            t.border,
            "gpui-component/input/token.rs:53",
        ))
        .color(claim(
            "token bg, selected",
            "selection",
            t.selection,
            "gpui-component/input/token.rs:50",
        ))
        .color(claim(
            "token text",
            "foreground",
            t.foreground,
            "gpui-component/input/token.rs:56",
        ))
        .config("token radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("token", "an InputToken chip the Input's token renderer draws, one line tall, gap_1 and px_1 inside a 1px border (input/token.rs, InputToken)")
        .instance("token", "@native-theme, a mention inserted once when the state is built (gpui-base/input/base/inline_tokens.rs, InlineToken::new)")
}
