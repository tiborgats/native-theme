//! The Layout page.

use gpui::{
    Context, IntoElement, ParentElement, SharedString, Styled, Window, div, prelude::*, px,
};
use gpui_component::{
    ActiveTheme, IconName, StyledExt,
    accordion::Accordion,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonVariants},
    carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPagination,
        CarouselPaginationItem, CarouselPrevious,
    },
    collapsible::Collapsible,
    form::{self, Field},
    group_box::GroupBoxVariants,
    h_flex,
    input::Input,
    label::Label,
    scroll::ScrollableElement,
    separator::Separator,
    setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    stepper::{Stepper, StepperItem},
    v_flex,
};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::support::{
    CAROUSEL_SLIDES, NativeStyled, STEPPER_STEPS, format_font_info, ghost_hover_fill, layout_value,
    native_geometry, native_group_box, native_icon, section, with_accordion_title_style, with_gap,
    with_padding,
};
use crate::{PROBE_CAROUSEL_LAST, PROBE_SETTINGS_ROW, PROBE_STEPPER, Page, probe};

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Layout
    // -----------------------------------------------------------------------
    pub(crate) fn render_layout_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let accordion_title_style = native_geometry(cx, geometry::accordion_title);
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
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
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // The four layout accessors, applied rather than printed: the outer
            // box takes the platform's window margin, the row inside it the
            // container margin and the widget gap, and the two rows are
            // separated by the section gap.
            .child(section("Layout spacing (the four LayoutTheme accessors)"))
            .child(
                div()
                    .id("tt-layout-spacing")
                    .child(
                        with_padding(
                            with_gap(
                                v_flex().demo_frame(cx),
                                section_gap,
                            ),
                            window_margin,
                        )
                        .child(
                            with_padding(
                                with_gap(
                                    h_flex().demo_frame(cx),
                                    widget_gap,
                                ),
                                container_margin,
                            )
                            .child(Button::new("ls-a").native(cx, geometry::button).label("One"))
                            .child(Button::new("ls-b").native(cx, geometry::button).label("Two"))
                            .child(Button::new("ls-c").native(cx, geometry::button).label("Three")),
                        )
                        .child(Label::new(SharedString::from(spacing_summary)).text_sm()),
                    )
                    .on_hover(self.hover_info(&fi, "Layout spacing", &[("border", "border", t.border, "showcase")], &[], &[("receivers", "none a theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no field behind it (theme/mod.rs, spacing_tokens), and its one reader, a Dialog's viewport margin, gets that default (dialog/dialog.rs, Dialog). So these four are the application's to apply (geometry.rs §9.5)")])),
            )
            // Dividers
            .child(section("Separator (solid / dashed / labeled)"))
            .child(
                div()
                    .id("tt-layout-divider")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(Separator::horizontal())
                            .child(Separator::horizontal().label("Section Break"))
                            .child(Separator::horizontal_dashed()),
                    )
                    .on_hover(self.hover_info(&fi, "Separator", &[("line", "border", t.border, "gpui-component/separator.rs:128"), ("label bg", "background", t.background, "gpui-component/separator.rs:149"), ("label text", "muted_foreground", t.muted_foreground, "gpui-component/separator.rs:150")], &[], &[("thickness", "Tier U, not an absence: the platform states separator.line_width and the model carries it. Upstream draws the line on an inner absolutely-positioned div at px(1.) and applies the caller's refinement to the outer container instead, so nothing reaches the line (separator.rs, Separator::render_base)")])),
            )
            // GroupBox as container
            .child(section("GroupBox as Layout Container"))
            .child(
                div()
                    .id("tt-layout-groupbox")
                    .child(
                        native_group_box(cx)
                        .title("Contained Content")
                        .fill()
                        .child(
                            v_flex()
                                .gap_2()
                                .child(
                                    Label::new(
                                        "GroupBox can wrap any content as a visual container.",
                                    )
                                    .text_sm(),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Button::new("gb-1").native(cx, geometry::button).label("Action A"))
                                        .child(Button::new("gb-2").native(cx, geometry::button).label("Action B").primary()),
                                ),
                        ),
                    )
                    .on_hover(self.hover_info(&fi, "GroupBox (layout)", &[("fill bg", "group_box", t.group_box, "gpui-component/group_box.rs:134"), ("text", "group_box_foreground", t.group_box_foreground, "gpui-component/group_box.rs:157"), ("title", "muted_foreground", t.muted_foreground, "gpui-component/group_box.rs:147")], &[("geometry", "geometry::group_box_content: card.border.padding_*, corner_radius, line_width, color -- refined last, so the radius and the edge drawn are the card's, not upstream's radius (group_box.rs, GroupBox)".to_string())], &[])),
            )
            // Scrollable area demo
            .child(section("Scrollable Area (visible scrollbar)"))
            .child(
                div()
                    .id("tt-scrollbar")
                    .occlude()
                    .child(
                        div()
                            .id("scroll-demo-outer")
                            .h(px(150.0))
                            .w_full()
                            .demo_frame(cx)
                            .overflow_y_scrollbar()
                            .native(cx, geometry::scrollbar_gutter)
                            .child(v_flex().gap_2().p_3().children((0..20).map(|i| {
                                Label::new(SharedString::from(format!(
                                    "Scrollable item #{} - demonstrates scrollbar theming",
                                    i + 1
                                )))
                                .text_sm()
                            }))),
                    )
                    .on_hover(self.hover_info(&fi, "Scrollbar", &[("track", "scrollbar", t.scrollbar, "gpui-component/theme/mod.rs:307"), ("thumb", "scrollbar_thumb", t.scrollbar_thumb, "gpui-component/theme/mod.rs:312"), (
                                "thumb hover",
                                "scrollbar_thumb_hover",
                                t.scrollbar_thumb_hover, "gpui-component/theme/mod.rs:323")], &[
                            ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                            (
                                "show mode",
                                format!("scrollbar_mode: {:?}", t.scrollbar_mode),
                            ),
                        ("geometry", "geometry::scrollbar_gutter on the element a scroll container scrolls: scrollbar.groove_width as right padding where scrollbar.overlay_mode is false, because gpui-component overlays the bar on the scroll area instead of putting it beside the content. The track width and the minimum thumb length are the platform's too, written onto gpui-base by base_layer::scrollbar_styles".to_string())], &[
                        ])),
            )
            // Accordion
            .child(section("Accordion"))
            .child(
                div()
                    .id("tt-accordion")
                    .child(
                        Accordion::new("acc-1")
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("What is native-theme?")
                                    .open(true)
                                    .child(
                                        Label::new(
                                            "A cross-platform theme abstraction that reads OS \
                                             settings and maps them to toolkit-specific themes.",
                                        )
                                        .text_sm(),
                                    )
                            })
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("Supported toolkits")
                                    .child(
                                        Label::new("gpui-component, iced, egui, and more planned.")
                                            .text_sm(),
                                    )
                            })
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("How many presets?")
                                    .child(
                                        // Counted from the list itself, so the
                                        // answer cannot go stale again.
                                        Label::new(SharedString::from(format!(
                                            "{} built-in theme presets covering major OS styles.",
                                            native_theme::theme::Theme::list_presets().len()
                                        )))
                                        .text_sm(),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "Accordion", &[("bg", "accordion", t.accordion, "gpui-component/accordion.rs:371"), ("border", "border", t.border, "showcase"), ("text", "foreground", t.foreground, "gpui-component/accordion.rs:305"), ("secondary text", "muted_foreground", t.muted_foreground, "gpui-component/accordion.rs:329")], &[("border-radius", format!("radius: {}px", t.radius.as_f32())), ("header height", "geometry::accordion_title: expander.header_height".to_string())], &[
                            ("padding", "inner (Tier U)"),
                            ("animation", "reads the theme's spring_control (accordion.rs, Accordion). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's"),
                        ])),
            )
            // Collapsible
            .child(section("Collapsible"))
            .child(
                div()
                    .id("tt-collapsible")
                    .child(
                        Collapsible::new()
                            .open(collapsible_open)
                            .child(
                                Button::new("coll-toggle")
                                    .label(if collapsible_open {
                                        "Click to collapse"
                                    } else {
                                        "Click to expand"
                                    })
                                    .ghost()
                                    .icon(if collapsible_open {
                                        IconName::ChevronDown
                                    } else {
                                        IconName::ChevronRight
                                    })
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.collapsible_open = !this.collapsible_open;
                                    })),
                            )
                            .content(
                                v_flex().p_3().child(
                                    Label::new("This content is shown when collapsible is open.")
                                        .text_sm(),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Collapsible", &[("toggle text", "secondary_foreground", t.secondary_foreground, "gpui-component/button/button.rs:964"), ("toggle hover (half alpha in dark mode)", "accent", ghost_hover_fill(&t), "gpui-component/button/button.rs:1125-1131"), ("content text", "foreground", t.foreground, "gpui-component/label.rs:211")], &[], &[("fill and edge", "none: a Collapsible only shows or hides its content and paints nothing of its own (collapsible.rs, Collapsible). The accordion fill and border the panel had claimed are an Accordion's; the toggle is this demo's ghost Button"), ("animation", "reads the theme's spring_control (collapsible.rs, Collapsible), the same writable Theme::motion the Accordion uses; the connector leaves it at its default because native-theme models no motion")])),
            )
            // Carousel
            .child(section("Carousel"))
            .child(
                div()
                    .id("tt-carousel")
                    // The slide controls are positioned outside the frame, so
                    // the section leaves a button's width on either side.
                    .px_16()
                    .child(
                        Carousel::new("carousel", &self.carousel_state)
                            .w(px(360.0))
                            .child(
                                CarouselContent::new(&self.carousel_state)
                                    .h(px(120.0))
                                    .children(CAROUSEL_SLIDES.iter().enumerate().map(
                                        |(ix, (title, body))| {
                                            CarouselItem::new(
                                                ("carousel-slide", ix),
                                                ix,
                                                &self.carousel_state,
                                            )
                                            .child(
                                                v_flex()
                                                    .size_full()
                                                    .justify_center()
                                                    .gap_1()
                                                    .p_4()
                                                    .rounded(t.radius)
                                                    .bg(t.muted)
                                                    .child(Label::new(*title).font_semibold())
                                                    .child(
                                                        Label::new(*body)
                                                            .text_sm()
                                                            .text_color(t.muted_foreground),
                                                    ),
                                            )
                                        },
                                    )),
                            )
                            .child(CarouselPagination::new().children(
                                (0..CAROUSEL_SLIDES.len()).map(|ix| {
                                    let dot = CarouselPaginationItem::new(
                                        ("carousel-page", ix),
                                        ix,
                                        &self.carousel_state,
                                    )
                                    .child(SharedString::from((ix + 1).to_string()));
                                    // The last dot is the self-test's way into
                                    // the carousel: the prev/next controls
                                    // place themselves absolutely outside the
                                    // frame, so a wrapper around one of those
                                    // would take it out of the flow.
                                    match ix == CAROUSEL_SLIDES.len() - 1 {
                                        true => probe(PROBE_CAROUSEL_LAST, dot).into_any_element(),
                                        false => dot.into_any_element(),
                                    }
                                }),
                            ))
                            // The carousel's own slide controls. They take the
                            // same state as the viewport and position
                            // themselves outside the frame
                            // (`carousel/carousel.rs:757-768`), so they belong
                            // to the carousel rather than beside it.
                            .child(CarouselPrevious::new(&self.carousel_state))
                            .child(CarouselNext::new(&self.carousel_state)),
                    )
                    .on_hover(self.hover_info(&fi, "Carousel", &[("slide text", "foreground", t.foreground, "gpui-component/label.rs:211"), ("slide caption", "muted_foreground", t.muted_foreground, "gpui-component/label.rs:171"), ("focus ring", "ring", t.ring, "gpui-component/styled.rs:187")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("slide fill", "none: a Carousel paints no slide background. The one theme colour it draws is the focus ring, on an overlay the size of the frame while focus is visible (carousel/carousel.rs, Carousel)"),
                            ("snap motion", "Theme::motion's spring_move (carousel/carousel.rs, spring_move); ResolvedTheme has no motion field"),
                            ("reduced motion", "gpui's App::reduce_motion, forwarded by apply_system_theme -- the spring jumps straight to its target (gpui-base/motion.rs, spring)"),
                            ("slide controls", "outline Buttons the widget builds itself, disabled at the ends (carousel/carousel.rs, carousel_control)"),
                        ])),
            )
            // GroupBox variants
            .child(section("GroupBox (3 variants)"))
            .child(
                div()
                    .id("tt-groupbox")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                native_group_box(cx)
                                    .title("Default")
                                    .w(px(180.0))
                                    .child(Label::new("Default style").text_sm()),
                            )
                            .child(
                                native_group_box(cx)
                                    .title("Filled")
                                    .fill()
                                    .w(px(180.0))
                                    .child(Label::new("Filled background").text_sm()),
                            )
                            .child(
                                native_group_box(cx)
                                    .title("Outline")
                                    .outline()
                                    .w(px(180.0))
                                    .child(Label::new("Outlined border").text_sm()),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "GroupBox", &[("fill bg", "group_box", t.group_box, "gpui-component/group_box.rs:134"), ("text", "group_box_foreground", t.group_box_foreground, "gpui-component/group_box.rs:157"), ("title", "muted_foreground", t.muted_foreground, "gpui-component/group_box.rs:147")], &[("geometry", "geometry::group_box_content: card.border.padding_*, corner_radius, line_width, color -- refined last, so the radius and the edge drawn are the card's, not upstream's radius (group_box.rs, GroupBox)".to_string())], &[])),
            )
            // Breadcrumb (with page navigation)
            .child(section("Breadcrumb (click to navigate pages)"))
            .child(
                div()
                    .id("tt-breadcrumb")
                    .child(
                        Breadcrumb::new()
                            .child(BreadcrumbItem::new("Buttons").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Buttons, cx),
                            )))
                            .child(BreadcrumbItem::new("Inputs").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Inputs, cx),
                            )))
                            .child(BreadcrumbItem::new("Data").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Data, cx),
                            )))
                            .child(BreadcrumbItem::new("Feedback").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Feedback, cx),
                            )))
                            .child(BreadcrumbItem::new("Layout")),
                    )
                    .on_hover(self.hover_info(&fi, "Breadcrumb", &[("last item", "foreground", t.foreground, "gpui-component/breadcrumb.rs:102"), ("non-last + separators", "muted_foreground", t.muted_foreground, "gpui-component/breadcrumb.rs:101")], &[], &[
                            ("separator icon", "a ChevronRight built inline with no setter to replace it (breadcrumb.rs, Breadcrumb) -- unlike an Alert's icon, which Alert::icon takes"),
                            ("spacing", "gap_1p5 (breadcrumb.rs, Breadcrumb) -- rems again -- and applied before the caller's refinement. native-theme states no breadcrumb widget. Our gap"),
                        ])),
            )
            // Stepper
            .child(section(format!(
                "Stepper (step {} of {}: one completed, one current, one pending)",
                self.step + 1,
                STEPPER_STEPS.len()
            )))
            .child(
                div()
                    .id("tt-stepper")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(480.0))
                            .child(probe(
                                PROBE_STEPPER,
                                Stepper::new("stepper-1")
                                    .selected_index(self.step)
                                    .items(STEPPER_STEPS.iter().map(|(label, icon)| {
                                        // The indicator is a circle
                                        // (stepper/trigger.rs:118-123) around
                                        // the icon it is given, which keeps its
                                        // size (`:138-139`).
                                        StepperItem::new()
                                            .icon(native_icon(
                                                cx,
                                                icon.clone(),
                                                geometry::icon_size_small,
                                            ))
                                            .child(Label::new(*label).text_sm())
                                    }))
                                    .on_click(cx.listener(|this, step: &usize, _w, cx| {
                                        this.step = *step;
                                        cx.notify();
                                    })),
                            ))
                            .child(
                                Stepper::new("stepper-vertical")
                                    .vertical()
                                    .selected_index(self.step)
                                    .items(STEPPER_STEPS.iter().map(|(label, _)| {
                                        StepperItem::new().child(Label::new(*label).text_sm())
                                    }))
                                    .on_click(cx.listener(|this, step: &usize, _w, cx| {
                                        this.step = *step;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Stepper", &[("completed / current", "primary", t.primary, "gpui-component/stepper/trigger.rs:133"), ("separator", "border", t.border, "gpui-component/stepper/item.rs:263"), ("passed separator", "primary", t.primary, "gpui-component/stepper/item.rs:264"), ("completed text", "primary_foreground", t.primary_foreground, "gpui-component/stepper/trigger.rs:134"), ("pending", "secondary", t.secondary, "gpui-component/stepper/trigger.rs:126"), ("pending text", "secondary_foreground", t.secondary_foreground, "gpui-component/stepper/trigger.rs:131"), ("pending hover", "secondary_hover", t.secondary_hover, "gpui-component/stepper/trigger.rs:128")], &[("icon size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                            ("indicator size", "24px for Size::Medium (stepper/item.rs, StepperItem::render icon_size)"),
                            ("separator", "drawn by the item, absolute (stepper/item.rs: StepperItem::render builds it, StepperSeparator::render positions it)"),
                        ])),
            )
            // Form / Field
            .child(section("Form / Field (horizontal layout)"))
            .child(
                div()
                    .id("tt-form")
                    .child(
                        form::Form::horizontal()
                            .label_width(px(100.0))
                            .child(
                                Field::new().label("Name").required(true).child(
                                    Input::new(&self.form_name_state)
                                        .native(cx, geometry::input),
                                ),
                            )
                            .child(
                                Field::new()
                                    .label("Email")
                                    .description("We will never share your email.")
                                    .child(
                                        Input::new(&self.form_email_state)
                                            .native(cx, geometry::input),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Form / Field", &[("label", "foreground", t.foreground, "gpui-component/root.rs:596"), ("description", "muted_foreground", t.muted_foreground, "gpui-component/form/field.rs:338"), ("required marker", "danger", t.danger, "gpui-component/form/field.rs:315")], &[("geometry", "geometry::input on each field's input: input.min_height (control height), border.corner_radius, line_width, input.font. The Field wrapper takes none -- the model has no form theme".to_string())], &[
                            ("layout", "horizontal/vertical"),
                            ("label width", "configurable"),
                        ])),
            )
            // Settings
            .child(section("Settings (page with field types)"))
            .child(
                div()
                    .id("tt-settings")
                    .debug_selector(|| "settings-frame".into())
                    .h(px(320.0))
                    .w_full()
                    .occlude()
                    .demo_frame(cx)
                    .overflow_y_scroll()
                    .child(
                        Settings::new("settings-demo")
                            .sidebar_width(px(140.0))
                            .page(
                                SettingPage::new("Appearance")
                                    .description("Customize the look and feel")
                                    .default_open(true)
                                    .group(
                                        SettingGroup::new()
                                            .native(cx, geometry::scrollbar_gutter)
                                            .title("Theme")
                                            .item(
                                                SettingItem::new(
                                                    "Dark Mode",
                                                    SettingField::switch(
                                                        |_cx| false,
                                                        |_val, _cx| {},
                                                    ),
                                                )
                                                .description("Toggle dark appearance"),
                                            )
                                            // The row probe is a whole-row item
                                            // (setting/item.rs, SettingItem::render):
                                            // a field's slot is only as wide as its
                                            // content, so it would not span the row.
                                            .item(SettingItem::render(|_, _, _| {
                                                div()
                                                    .w_full()
                                                    .h(px(1.))
                                                    .debug_selector(|| PROBE_SETTINGS_ROW.into())
                                            }))
                                            .item(SettingItem::new(
                                                "Accent Color",
                                                SettingField::dropdown(
                                                    vec![
                                                        ("blue".into(), "Blue".into()),
                                                        ("green".into(), "Green".into()),
                                                        ("red".into(), "Red".into()),
                                                    ],
                                                    |_cx| "blue".into(),
                                                    |_val, _cx| {},
                                                ),
                                            )),
                                    )
                                    .group(
                                        SettingGroup::new()
                                            .native(cx, geometry::scrollbar_gutter)
                                            .title("Editor")
                                            .item(SettingItem::new(
                                                "Font Size",
                                                SettingField::input(
                                                    |_cx| "14".into(),
                                                    |_val, _cx| {},
                                                ),
                                            ))
                                            .item(SettingItem::new(
                                                "Word Wrap",
                                                SettingField::checkbox(|_cx| true, |_val, _cx| {}),
                                            )),
                                    ),
                            )
                            .page(
                                SettingPage::new("Keyboard")
                                    .description("Keyboard shortcuts and input")
                                    .group(
                                        SettingGroup::new()
                                            .native(cx, geometry::scrollbar_gutter)
                                            .title("Shortcuts")
                                            .item(SettingItem::new(
                                                "Vim Mode",
                                                SettingField::switch(|_cx| false, |_val, _cx| {}),
                                            )),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Settings", &[("sidebar", "sidebar", t.sidebar, "gpui-component/sidebar/mod.rs:413"), ("border", "border", t.border, "gpui-component/setting/page.rs:185")], &[("geometry", "geometry::scrollbar_gutter on each SettingGroup: scrollbar.groove_width as right padding where scrollbar.overlay_mode is false, so the page's own scrollbar, laid over the body's right edge, does not cover the rows".to_string())], &[("groups", "no fill and no edge: a Settings page uses GroupBoxVariant::Normal unless Settings::with_group_variant picks Fill or Outline, so group_box paints nothing here (setting/settings.rs, with_group_variant)"), 
                            ("fill", "none of its own: nothing under setting/ sets a background, so a Settings page shows the window's (gpui-component setting/)"),
                            ("descriptions", "muted_foreground, set per item, group and page (setting/item.rs, setting/group.rs, setting/page.rs)"),
                            ("fields", "switch, checkbox, input, number input, dropdown, or an element of the application's own (setting/fields/mod.rs, SettingFieldType)"),
                            ("layout", "sidebar + pages"),
                            ("scrollbar", "gpui-component lays the page's scrollbar over the body's right edge (setting/page.rs, SettingPage) and reserves only 1rem; this demo pads each group by the platform's groove width instead (setting/group.rs, SettingGroup). The page body takes no refinement -- Tier U"),
                        ])),
            )
    }
}
