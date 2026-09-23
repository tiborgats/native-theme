//! What the Layout page's widgets report about themselves (spec §3.4).

use gpui::Pixels;
use gpui_component::theme::Theme;

use super::{WidgetInfo, chrome::ghost_hover, claim, px_text};
use crate::demo::{GroupBoxKind, SeparatorKind, SpacingBox, StepperKind};
use crate::support::layout_value;

/// One of the Layout page's two spacing boxes, `kind`, padded by `padding`
/// and spacing its children by `gap`, each `None` where the platform states
/// none. Their geometry lines are recorded where `demo::spacing_box` applies
/// the values.
pub fn spacing_box(
    t: &Theme,
    kind: SpacingBox,
    padding: Option<Pixels>,
    gap: Option<Pixels>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Layout spacing");
    let info = match kind {
        SpacingBox::Window => info
            .variant("window margin, section gap")
            .color(claim("frame", "border", t.border, "showcase"))
            .config("window_margin", layout_value(padding))
            .config("section_gap", layout_value(gap)),
        SpacingBox::Container => info
            .variant("container margin, widget gap")
            .color(claim("frame", "border", t.border, "showcase"))
            .config("container_margin", layout_value(padding))
            .config("widget_gap", layout_value(gap)),
    };
    let info = info.not_themeable("receivers", "none a theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no field behind it (theme/mod.rs, spacing_tokens), and its one reader, a Dialog's viewport margin, gets that default (dialog/dialog.rs, Dialog). So these four are the application's to apply (geometry.rs §9.5)");
    match kind {
        SpacingBox::Window => info.instance(
            "holds",
            "the row below and the summary under it: the window margin is the padding inside this frame, and the section gap the space between the two",
        ),
        SpacingBox::Container => info.instance(
            "holds",
            "three Default Buttons, which report themselves: the container margin is the padding inside this frame, and the widget gap the space between the Buttons",
        ),
    }
    .instance(
        "frame",
        "the showcase's own frame, not a widget of gpui-component's: Theme::border at the platform's border line width, rounded with Theme::radius",
    )
}

/// A `Separator` of `kind`.
pub fn separator(t: &Theme, kind: SeparatorKind) -> WidgetInfo {
    let info = WidgetInfo::new("Separator")
        .variant(kind.name())
        .color(claim(
            "line",
            "border",
            t.border,
            "gpui-component/separator.rs:128",
        ))
        .not_themeable(
            "thickness",
            "Tier U, not an absence: the platform states separator.line_width and the model carries it. Upstream draws the line on an inner absolutely-positioned div at px(1.) and applies the caller's refinement to the outer container instead, so nothing reaches the line (separator.rs, Separator::render_base)",
        );
    match kind {
        SeparatorKind::Vertical | SeparatorKind::Horizontal => info,
        SeparatorKind::Labelled(label) => info
            .color(claim(
                "label bg",
                "background",
                t.background,
                "gpui-component/separator.rs:149",
            ))
            .color(claim(
                "label text",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/separator.rs:150",
            ))
            .not_themeable(
                "label",
                "text_xs with px_2 / py_1 -- rems, so the platform's font -- on a box the caller's refinement does not reach: the Separator refines its outer container and builds the label as a child after it (separator.rs, Separator::render)",
            )
            .instance("label", label),
        SeparatorKind::Dashed => info.not_themeable(
            "dashes",
            "4px dashes with 2px gaps, stroked 1px wide -- literals (separator.rs, Separator::render_dashed)",
        ),
    }
}

/// A `GroupBox` of `kind` titled `title`. `styled` is whether
/// `geometry::group_box_content` refined its content, whose line is
/// recorded where `demo::group_box` applies the builder. The caller adds
/// what the content is.
pub fn group_box(t: &Theme, kind: GroupBoxKind, styled: bool, title: &str) -> WidgetInfo {
    let info = WidgetInfo::new("GroupBox")
        .variant(kind.name())
        .color(claim(
            "title",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/group_box.rs:147",
        ))
        // A Label would paint foreground over this on its own element
        // (label.rs:211), so the content's text is plain text.
        .color(claim(
            "content text",
            "group_box_foreground",
            t.group_box_foreground,
            "gpui-component/group_box.rs:157",
        ));
    // (fill, edge) per variant (group_box.rs:132-136).
    let info = match kind {
        GroupBoxKind::Normal => info,
        GroupBoxKind::Fill => info.color(claim(
            "bg",
            "group_box",
            t.group_box,
            "gpui-component/group_box.rs:134",
        )),
        GroupBoxKind::Outline if styled => info,
        GroupBoxKind::Outline => info.color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/group_box.rs:135",
        )),
    };
    let info = match (kind, styled) {
        (_, true) => info.config(
            "edge",
            "card.border.color at card.border.line_width, on every variant: geometry::group_box_content sets both on the content, which the GroupBox refines last (group_box.rs, GroupBox) -- over an Outline's own border colour, and on a Normal or Fill box, which draws no edge of its own",
        ),
        (GroupBoxKind::Outline, false) => info
            .config("border-radius", format!("radius: {}px", t.radius.as_f32())),
        (GroupBoxKind::Normal | GroupBoxKind::Fill, false) => info
            .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
            .not_themeable(
                "edge",
                "none: only the Outline variant draws a border (group_box.rs, GroupBox)",
            ),
    };
    let info = match kind {
        GroupBoxKind::Normal => info.not_themeable(
            "fill",
            "none: the Normal variant paints no background (group_box.rs, GroupBox)",
        ),
        GroupBoxKind::Fill | GroupBoxKind::Outline => info,
    };
    info.not_themeable(
        "gaps",
        "gap_3 between the title and the content on a Fill or Outline box and gap_4 on a Normal one -- rems, so the platform's font -- and settable: the GroupBox refines its outer box after them (group_box.rs, GroupBox)",
    )
    .instance("title", title.to_string())
}

/// The `Accordion`, drawn while gpui's `reduce_motion` is as given. Its
/// title rows' geometry line is recorded where `demo::accordion` applies
/// the builder.
pub fn accordion(t: &Theme, reduce_motion: bool) -> WidgetInfo {
    WidgetInfo::new("Accordion")
        .color(claim(
            "bg",
            "accordion",
            t.accordion,
            "gpui-component/accordion.rs:371",
        ))
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/accordion.rs:110",
        ))
        .color(claim(
            "line between items",
            "border",
            t.border,
            "gpui-component/accordion.rs:374",
        ))
        .color(claim(
            "open item's title",
            "foreground",
            t.foreground,
            "gpui-component/accordion.rs:305",
        ))
        // A closed item's title row sets no colour (accordion.rs:305 is
        // the open one's), so it takes the colour the showcase sets on its
        // window.
        .color(claim(
            "closed item's title, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "chevron",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/accordion.rs:329",
        ))
        .config(
            "border-radius",
            format!("radius_lg: {}px", t.radius_lg.as_f32()),
        )
        .not_themeable("padding", "per-Size literals, py_2 / px_3 on the title row and pb_2 / px_3 under the content at the default Size -- rems -- each followed by the caller's title_style or content_style (accordion.rs, AccordionItem::render), so both are reachable. geometry::accordion_title sets only the title row's height")
        .not_themeable("chevron", "a ChevronDown built inline at XSmall in muted_foreground, with no setter (accordion.rs, AccordionItem::render), so expander.arrow_icon_size and expander.arrow_color have no receiver -- Tier U")
        .not_themeable("animation", "reads the theme's spring_control (accordion.rs, AccordionItem). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so an item's content opens and closes at once (gpui-base/motion.rs, spring)"
        } else {
            "an item's content springs open and shut on spring_control; the chevron turns at once (accordion.rs, AccordionItem::render)"
        })
        .instance("items", "three, the first open. Accordion::item takes a closure over an AccordionItem that the Accordion builds and renders itself (accordion.rs, Accordion::item), so an item cannot be wrapped: the Accordion reports for its items and their titles. Each answer is a Label that reports itself")
}

/// The `Collapsible`, `open` or not.
pub fn collapsible(open: bool) -> WidgetInfo {
    WidgetInfo::new("Collapsible")
        .variant(if open { "open" } else { "closed" })
        .not_themeable("fill and edge", "none: a Collapsible only shows or hides its content and paints nothing of its own (collapsible.rs, Collapsible). The toggle is this demo's ghost Button and the content a Label, which report themselves")
        .instance("animation", "none: a Collapsible springs its reveal on the theme's spring_control only when it is given a motion_id (collapsible.rs, Collapsible::render), and this one is not, so its content appears and disappears at once")
}

/// The Collapsible's toggle: an upstream Ghost `Button` reading `label`,
/// its chevron pointing down while the Collapsible is `open`.
pub fn collapsible_toggle(t: &Theme, open: bool, label: &str) -> WidgetInfo {
    WidgetInfo::new("Button")
        .variant("Ghost, icon")
        .color(claim(
            "text and icon",
            "secondary_foreground",
            t.secondary_foreground,
            "gpui-component/button/button.rs:964",
        ))
        .color(ghost_hover(t))
        .color(claim(
            "text and icon on hover",
            "accent_foreground",
            t.accent_foreground,
            "gpui-component/button/button.rs:1141",
        ))
        .color(claim(
            "pressed",
            "button_active",
            t.button_active,
            "gpui-component/button/button.rs:1180",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable(
            "fill",
            "none: a Ghost Button is transparent until hovered (button/button.rs, ButtonVariant::hovered)",
        )
        .instance("variant", "gpui-component's own .ghost(), not native_theme_gpui::variants::ghost_button: it hovers with the item-highlight pair (button/button.rs, ButtonVariant::hovered Ghost arm)")
        .instance("label", label.to_string())
        .instance(
            "icon",
            if open {
                "ChevronDown, while the content shows"
            } else {
                "ChevronRight, while the content is hidden"
            },
        )
}

/// The `Carousel`, drawn while gpui's `reduce_motion` is as given.
pub fn carousel(t: &Theme, reduce_motion: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Carousel");
    // Carousel::render hands its focus overlay to focus_ring_style
    // (carousel/carousel.rs:201), which only tints the border where the
    // theme turns the ring off (styled.rs:182-184) and otherwise also draws
    // the ring outside it at FOCUS_RING_OPACITY, 0.5 (styled.rs:12, :186-190).
    let info = if t.focus_ring {
        info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/styled.rs:187",
        ))
        .color(claim(
            "focus ring, ring at 50%",
            "ring",
            t.ring.alpha(0.5),
            "gpui-component/styled.rs:189",
        ))
    } else {
        info.color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/styled.rs:183",
        ))
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .config("focus_ring", format!("{}", t.focus_ring))
        .not_themeable("slide fill", "none: a Carousel paints no slide background. The one theme colour it draws is the focus ring, on an overlay the size of the frame while focus is visible (carousel/carousel.rs, Carousel)")
        .not_themeable("snap motion", "Theme::motion's spring_move (carousel/carousel.rs, spring_move); ResolvedTheme has no motion field")
        .not_themeable("reduced motion", "gpui's App::reduce_motion, forwarded by apply_system_theme -- the spring jumps straight to its target (gpui-base/motion.rs, spring)")
        .not_themeable("slide controls", "outline Buttons the widget builds itself, disabled at the ends (carousel/carousel.rs, carousel_control)")
        .instance("animation", if reduce_motion {
            "none: reduced motion is on, so a change of slide jumps straight to it (gpui-base/motion.rs, spring)"
        } else {
            "a change of slide springs the track to it on spring_move (carousel/carousel.rs, CarouselContent)"
        })
        .instance("controls", "the Previous and Next Buttons place themselves outside the frame, positioned against the Carousel (carousel/carousel.rs, carousel_control), so a wrapper around one would become what it is positioned against: they report through the Carousel. The slides and the page buttons report themselves")
}

/// Slide `index` of the Carousel, titled `title` over `caption`: the
/// showcase's own content in a `CarouselItem`.
pub fn carousel_slide(t: &Theme, index: usize, title: &str, caption: &str) -> WidgetInfo {
    WidgetInfo::new("CarouselItem")
        .variant(format!("slide {}", index + 1))
        .color(claim("slide bg", "muted", t.muted, "showcase"))
        .color(claim(
            "title",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:211",
        ))
        .color(claim(
            "caption",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("item", "a CarouselItem paints nothing of its own and only pads its slide from the one before (carousel/carousel.rs, CarouselItem): the fill, the rounding and the text colours are the showcase's")
        .instance("title", title.to_string())
        .instance("caption", caption.to_string())
}

/// The page button for slide `index` of the Carousel, `selected` while
/// that slide is shown: a compact XSmall Default `Button` the
/// `CarouselPaginationItem` builds.
pub fn carousel_page(t: &Theme, index: usize, selected: bool) -> WidgetInfo {
    let info = WidgetInfo::new("CarouselPaginationItem");
    // A selected Button takes its selected style and no hover or press
    // style (button/button.rs:662, :747-752).
    let info = if selected {
        info.variant("selected")
            .color(claim(
                "bg",
                "button_active",
                t.button_active,
                "gpui-component/button/button.rs:1242",
            ))
            .color(claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:949",
            ))
            .color(claim(
                "border",
                "input",
                t.input,
                "gpui-component/button/button.rs:1001",
            ))
            .not_themeable(
                "hover",
                "none while selected: a selected Button takes no hover or press style (button/button.rs, RenderOnce for Button)",
            )
    } else {
        info.color(claim(
            "bg",
            "button",
            t.button,
            "gpui-component/button/button.rs:935",
        ))
        .color(claim(
            "text",
            "button_foreground",
            t.button_foreground,
            "gpui-component/button/button.rs:949",
        ))
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/button/button.rs:1001",
        ))
        .color(claim(
            "hover",
            "button_hover",
            t.button_hover,
            "gpui-component/button/button.rs:1079",
        ))
        .color(claim(
            "active",
            "button_active",
            t.button_active,
            "gpui-component/button/button.rs:1163",
        ))
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("size", "a compact Button at Size::XSmall, CarouselPaginationItem's default: h_5, at least w_5, px_1 -- rems, so the platform's font (carousel/carousel.rs, CarouselPaginationItem::new; button/button.rs, RenderOnce for Button)")
        .instance(
            "slide",
            format!("{}: a click shows that slide", index + 1),
        )
}

/// A `Stepper` of `kind` with `count` steps, step `step` the current one
/// (0-based). A horizontal one's icons take `geometry::icon_size_small`
/// where a native theme is installed, whose line is recorded where
/// `demo::stepper` applies it.
pub fn stepper(t: &Theme, kind: StepperKind, step: usize, count: usize) -> WidgetInfo {
    // Steps up to the current one are checked (stepper/trigger.rs:106), so
    // there is always one; a pending one only before the last step. The
    // separator after step i is passed while i is before the current step
    // (stepper/item.rs:117).
    let pending = step + 1 < count;
    let passed = step > 0 && count > 1;
    // (on a checked step, on a pending one): the icon or the number the
    // indicator holds takes the indicator's text colour (icon.rs, Icon).
    let (checked_mark, pending_mark) = match kind {
        StepperKind::Icons => (
            claim(
                "icon, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:134",
            ),
            claim(
                "icon, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:131",
            ),
        ),
        StepperKind::Numbers => (
            claim(
                "number, completed / current",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/stepper/trigger.rs:134",
            ),
            claim(
                "number, pending",
                "secondary_foreground",
                t.secondary_foreground,
                "gpui-component/stepper/trigger.rs:131",
            ),
        ),
    };
    let info = WidgetInfo::new("Stepper")
        .variant(kind.name())
        .color(claim(
            "completed / current",
            "primary",
            t.primary,
            "gpui-component/stepper/trigger.rs:133",
        ))
        .color(checked_mark);
    let info = if pending {
        info.color(claim(
            "pending",
            "secondary",
            t.secondary,
            "gpui-component/stepper/trigger.rs:126",
        ))
        .color(pending_mark)
        .color(claim(
            "pending hover",
            "secondary_hover",
            t.secondary_hover,
            "gpui-component/stepper/trigger.rs:128",
        ))
        .color(claim(
            "pending pressed",
            "secondary_active",
            t.secondary_active,
            "gpui-component/stepper/trigger.rs:129",
        ))
        .color(claim(
            "separator",
            "border",
            t.border,
            "gpui-component/stepper/item.rs:263",
        ))
    } else {
        info
    };
    let info = if passed {
        info.color(claim(
            "passed separator",
            "primary",
            t.primary,
            "gpui-component/stepper/item.rs:264",
        ))
    } else {
        info
    };
    let info = info
        .not_themeable("indicator size", "24px for Size::Medium (stepper/item.rs, StepperItem::render icon_size)")
        .not_themeable("separator", "drawn by the item, absolute (stepper/item.rs: StepperItem::render builds it, StepperSeparator::render positions it)");
    let info = match kind {
        StepperKind::Icons => info.instance("indicator", "each step's icon, which keeps its own size inside the circle (stepper/trigger.rs, StepperTrigger)"),
        StepperKind::Numbers => info.instance("indicator", "no icon is given, so each step shows its number (stepper/trigger.rs, StepperTrigger)"),
    };
    info.instance(
        "steps",
        format!(
            "{count}; step {} is current, so {step} completed and {} pending. A click on a step makes it current, in both Steppers",
            step + 1,
            count.saturating_sub(step + 1)
        ),
    )
    .instance("items", "Stepper::item takes a StepperItem, which the Stepper numbers and renders itself (stepper/stepper.rs, Stepper::item), so a step cannot be wrapped: the Stepper reports for its steps. Each step's label is a Label that reports itself")
}

/// The `Breadcrumb`, whose items but the last show a page.
pub fn breadcrumb(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Breadcrumb")
        .color(claim(
            "items",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/breadcrumb.rs:101",
        ))
        .color(claim(
            "last item",
            "foreground",
            t.foreground,
            "gpui-component/breadcrumb.rs:102",
        ))
        .color(claim(
            "separators",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/breadcrumb.rs:144",
        ))
        .not_themeable("separator icon", "a ChevronRight built inline with no setter to replace it (breadcrumb.rs, Breadcrumb) -- unlike an Alert's icon, which Alert::icon takes")
        .not_themeable("spacing", "gap_1p5 (breadcrumb.rs, Breadcrumb) -- rems again -- and applied before the caller's refinement. native-theme states no breadcrumb widget. Our gap")
        .instance("items", "Buttons, Inputs, Data, Feedback and Layout: a click on one of the first four shows that page, and Layout, the last, is this page. Breadcrumb::child takes a BreadcrumbItem that the Breadcrumb renders itself (breadcrumb.rs, Breadcrumb::child), so an item cannot be wrapped: the Breadcrumb reports for its items")
}

/// The horizontal `Form` of the Layout page, its labels `label_width` wide,
/// whose Fields' Inputs report themselves.
pub fn form(t: &Theme, label_width: Pixels) -> WidgetInfo {
    WidgetInfo::new("Form")
        .variant("horizontal")
        // The label's box sets a size and a weight but no colour
        // (form/field.rs:296-301), so the label takes the colour the
        // showcase sets on its window.
        .color(claim(
            "label, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "description",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/form/field.rs:338",
        ))
        .color(claim(
            "required marker",
            "danger",
            t.danger,
            "gpui-component/form/field.rs:315",
        ))
        .not_themeable("fields", "the Field wrapper takes no geometry -- the model has no form theme. Each field's Input is refined by geometry::input and reports itself")
        .not_themeable("label style", "text_sm and font_medium on a box that sets no colour (form/field.rs, Field)")
        .instance("layout", "horizontal: Form::horizontal puts each label beside its field, and the description under the field (form/field.rs, Field)")
        .instance(
            "label width",
            format!(
                "{}px, the showcase's own through Form::label_width; upstream's default is 140px (form/field.rs, FieldProps)",
                px_text(label_width.as_f32())
            ),
        )
        .instance("fields", "Name, required, and Email, with a description. Form::child takes a Field (form/form.rs, Form::child), which the Form lays out itself, so a field cannot be wrapped: the Form reports for its fields")
}

/// The scrollable area, a column of Labels in a container gpui-component's
/// `overflow_y_scrollbar` scrolls. `styled` is whether a native theme is
/// installed, whose scrollbar the connector writes onto gpui-base; its
/// gutter's geometry line is recorded where `demo::scroll_area` applies the
/// builder.
pub fn scroll_area(t: &Theme, styled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Scrollbar")
        .color(claim(
            "track",
            "scrollbar",
            t.scrollbar,
            "gpui-component/theme/mod.rs:307",
        ))
        .color(claim(
            "thumb",
            "scrollbar_thumb",
            t.scrollbar_thumb,
            "gpui-component/theme/mod.rs:312",
        ))
        .color(claim(
            "thumb hover",
            "scrollbar_thumb_hover",
            t.scrollbar_thumb_hover,
            "gpui-component/theme/mod.rs:323",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .config(
            "show mode",
            format!("scrollbar_mode: {:?}", t.scrollbar_mode),
        );
    let info = if styled {
        info.config("track and thumb", "scrollbar.groove_width wide, the thumb scrollbar.thumb_width wide and at least scrollbar.min_thumb_length long, in scrollbar.track_color, thumb_color and thumb_hover_color -- the colours ThemeColor holds as scrollbar, scrollbar_thumb and scrollbar_thumb_hover -- written onto gpui-base by the connector after every rebuild (native-theme-gpui base_layer.rs, scrollbar_styles)")
    } else {
        info
    };
    info.instance(
        "content",
        "twenty Labels, each of which reports itself; the strip beside them is the scrollbar's",
    )
}

/// The Layout page's `Settings` sample. Its geometry line is recorded where
/// `demo::settings` applies the builder to its groups.
pub fn settings(t: &Theme) -> WidgetInfo {
    super::chrome::settings(t, "two pages")
        .color(claim(
            "group title",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/group_box.rs:147",
        ))
        .not_themeable("fill", "none of its own: nothing under setting/ sets a background, so a Settings page shows the window's (gpui-component setting/)")
        .not_themeable("descriptions", "muted_foreground, set per item, group and page (setting/item.rs, setting/group.rs, setting/page.rs)")
        .not_themeable("layout", "the label above the field wherever the page is at most 480px wide, and beside it where it is wider (setting/settings.rs, STACKED_LAYOUT_MAX_WIDTH)")
        .instance("pages", "Appearance, open, with the groups Theme and Editor, and Keyboard, with Shortcuts; the sidebar lists the pages")
        .instance("fields", "switch, checkbox, input, number input, dropdown, or an element of the application's own (setting/fields/mod.rs, SettingFieldType). This sample shows the first three and a dropdown, which upstream builds inside each SettingItem, so they report through the Settings")
}
