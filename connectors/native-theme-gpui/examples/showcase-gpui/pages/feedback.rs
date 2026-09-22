//! The Feedback tab.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, prelude::*, px};
use gpui_component::{
    ActiveTheme, IconName, Sizable, Size, WindowExt,
    alert::Alert,
    badge::Badge,
    button::Button,
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    h_flex,
    label::Label,
    marker::{Marker, MarkerContent, MarkerIcon, MarkerLoadingStyle, MarkerVariant},
    notification::Notification,
    progress::{Progress, ProgressCircle},
    shimmer::ShimmerText,
    skeleton::Skeleton,
    spinner::Spinner,
    tag::Tag,
    tooltip::Tooltip,
    v_flex,
};
use std::time::Duration;

use native_theme_gpui::{ActiveNativeTheme, geometry};

use crate::app::Showcase;
use crate::support::{
    NativeStyled, format_font_info, native_geometry, native_icon, native_value, refined, section,
    with_gap,
};
use crate::{PROBE_NOTIFICATION, probe};

impl Showcase {
    // -----------------------------------------------------------------------
    // Tab: Feedback
    // -----------------------------------------------------------------------
    pub(crate) fn render_feedback_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Alerts with icons
            .child(section("Alerts (all 4 variants with icons)"))
            .child(
                div()
                    .id("tt-alert-info")
                    .child(
                        Alert::info("alert-info", "This is an informational message.")
                            .title("Info"),
                    )
                    .on_hover(self.hover_info(&fi, "Alert (Info)", &[("color", "info", t.info, "gpui-component/alert.rs:29"), ("text", "info", t.info, "gpui-component/alert.rs:29"), ("border", "info", t.info, "gpui-component/alert.rs:49")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("tints", "the variant colour faded into transparent white: 4% of it for the fill and 30% for the edge -- one token, three strengths, so the swatches show the pure colour rather than what is painted (alert.rs, AlertVariant)"),
                            ("padding", "px literals per Size, but reachable: Alert is Styled and applies the caller's refinement after its own paddings (alert.rs, Alert::render). Its corner radius already comes from the theme (alert.rs, Alert::render radius). What is missing is a model -- native-theme states no alert widget, so there is nothing to carry. Our gap, not upstream's"),
                            ("icon", "the Info variant's default, not a fixed glyph: Alert::icon replaces it with any Icon (alert.rs, Alert::icon), and an Icon takes a path or raw SVG bytes (icon.rs, Icon::path), so a platform icon from this connector's loader can be handed to it"),
                            ("icon size", "size_4 / size_5 per Size (alert.rs, Alert::render) -- rems, so it follows the platform font. defaults.icon_sizes is in absolute px and there is no alert in the model to hang it on"),
                        ])),
            )
            .child(
                div()
                    .id("tt-alert-success")
                    .child(
                        Alert::success("alert-ok", "Operation completed successfully.")
                            .title("Success"),
                    )
                    .on_hover(self.hover_info(&fi, "Alert (Success)", &[("color", "success", t.success, "gpui-component/alert.rs:30"), ("text", "success", t.success, "gpui-component/alert.rs:30"), ("border", "success", t.success, "gpui-component/alert.rs:50")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("padding", "px literals per Size, but reachable: Alert is Styled and applies the caller's refinement after its own paddings (alert.rs, Alert::render). Its corner radius already comes from the theme (alert.rs, Alert::render radius). What is missing is a model -- native-theme states no alert widget, so there is nothing to carry. Our gap, not upstream's"),
                            ("icon", "the Success variant's default; Alert::icon replaces it (alert.rs, Alert::icon)"),
                        ])),
            )
            .child(
                div()
                    .id("tt-alert-warning")
                    .child(
                        Alert::warning("alert-warn", "Please review before proceeding.")
                            .title("Warning"),
                    )
                    .on_hover(self.hover_info(&fi, "Alert (Warning)", &[("color", "warning", t.warning, "gpui-component/alert.rs:31"), ("text", "warning", t.warning, "gpui-component/alert.rs:31"), ("border", "warning", t.warning, "gpui-component/alert.rs:51")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("padding", "px literals per Size, but reachable: Alert is Styled and applies the caller's refinement after its own paddings (alert.rs, Alert::render). Its corner radius already comes from the theme (alert.rs, Alert::render radius). What is missing is a model -- native-theme states no alert widget, so there is nothing to carry. Our gap, not upstream's"),
                            ("icon", "the Warning variant's default; Alert::icon replaces it (alert.rs, Alert::icon)"),
                        ])),
            )
            .child(
                div()
                    .id("tt-alert-error")
                    .child(
                        Alert::error("alert-err", "Something went wrong. Please try again.")
                            .title("Error"),
                    )
                    .on_hover(self.hover_info(&fi, "Alert (Error)", &[("color", "danger", t.danger, "gpui-component/alert.rs:32"), ("text", "danger", t.danger, "gpui-component/alert.rs:32"), ("border", "danger", t.danger, "gpui-component/alert.rs:52")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("padding", "px literals per Size, but reachable: Alert is Styled and applies the caller's refinement after its own paddings (alert.rs, Alert::render). Its corner radius already comes from the theme (alert.rs, Alert::render radius). What is missing is a model -- native-theme states no alert widget, so there is nothing to carry. Our gap, not upstream's"),
                            ("icon", "the Error variant's default; Alert::icon replaces it (alert.rs, Alert::icon)"),
                        ])),
            )
            // Progress
            .child(section("Progress Bars"))
            .child(
                div()
                    .id("tt-progress")
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
                            .child(refined(
                                Progress::new("progress-upload").value(73.0),
                                native_geometry(cx, geometry::progress).as_ref(),
                            ))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Processing").text_sm())
                                    .child(Label::new("45%").text_sm()),
                            )
                            .child(refined(
                                Progress::new("progress-processing").value(45.0),
                                native_geometry(cx, geometry::progress).as_ref(),
                            ))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Complete").text_sm())
                                    .child(Label::new("100%").text_sm()),
                            )
                            .child(refined(
                                Progress::new("progress-complete").value(100.0),
                                native_geometry(cx, geometry::progress).as_ref(),
                            )),
                    )
                    .on_hover(self.hover_info(&fi, "Progress", &[("bar", "progress_bar", t.progress_bar, "gpui-component/progress/progress.rs:86")], &[("geometry", "geometry::progress: progress_bar.track_height, border.corner_radius, min_width".to_string())], &[
                            ("animation", "not hardcoded upstream: the fill transitions over the theme's duration_normal and easing_move (progress/progress.rs, Progress). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's"),
                        ])),
            )
            // ProgressCircle
            .child(section("ProgressCircle (determinate and indeterminate)"))
            .child(
                div()
                    .id("tt-progress-circle")
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(
                                with_gap(h_flex(), widget_gap)
                                    .items_center()
                                    .child(ProgressCircle::new("progress-circle-73").value(73.0))
                                    .child(Label::new("73%").text_sm()),
                            )
                            .child(
                                with_gap(h_flex(), widget_gap)
                                    .items_center()
                                    .child(
                                        ProgressCircle::new("progress-circle-100")
                                            .value(100.0)
                                            .with_size(Size::Large),
                                    )
                                    .child(Label::new("100%").text_sm()),
                            )
                            .child({
                                // Indeterminate, it is a spinner drawn as an
                                // arc, so the platform's spinner diameter is
                                // the size it should take.
                                let circle =
                                    ProgressCircle::new("progress-circle-loading").loading(true);
                                match native_value(cx, geometry::spinner_size) {
                                    Some(size) => circle.with_size(size),
                                    None => circle,
                                }
                            })
                            .child(
                                Label::new("indeterminate")
                                    .text_sm()
                                    .text_color(t.muted_foreground),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "ProgressCircle", &[("arc", "progress_bar", t.progress_bar, "gpui-component/progress/progress_circle.rs:176"), ("track", "progress_bar", t.progress_bar, "gpui-component/progress/progress_circle.rs:176")], &[("indeterminate size", "geometry::spinner_size: spinner.diameter -- drawn at 75% of it, because a ProgressCircle scales a Size::Size by 0.75 where a Spinner takes it whole (progress/progress_circle.rs, ProgressCircle)".to_string())], &[
                            ("determinate size", "size_2 to size_5 per Size, rems, so it follows the platform font; the model carries no circular-progress diameter"),
                            ("track opacity", "the same progress_bar colour at 20%: the model states one bar colour, and the track is derived from it (progress/progress_circle.rs, ProgressCircle::render_circle)"),
                            ("stroke width", "15% of the diameter, capped at 5px (progress/progress_circle.rs, ProgressCircle::render_circle stroke_width)"),
                        ])),
            )
            // Spinners
            .child(section("Spinner (3 sizes)"))
            .child(
                div()
                    .id("tt-spinner")
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
                                    .child(Spinner::new().with_size(
                                        cx.native_theme()
                                            .and_then(|nt| nt.native(cx))
                                            .map_or(Size::Medium, geometry::spinner_size),
                                    ))
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
                    .on_hover(self.hover_info(
                        &fi,
                        "Spinner",
                        &[],
                        &[("size", "Small/Large per Size enum; Medium via geometry::spinner_size (spinner.diameter)".to_string())],
                        &[
                            ("animation speed", "a 0.8s private field with no setter, and not one of the theme's motion tokens (spinner.rs, Spinner) -- nothing to write, upstream or here"),
                        ],
                    )),
            )
            // Skeleton
            .child(section("Skeleton Placeholders"))
            .child(
                div()
                    .id("tt-skeleton")
                    .child(
                        v_flex()
                            .gap_2()
                            .w(px(360.0))
                            // The placeholder's rounding is the theme's, not a
                            // number of its own; the block keeps the larger of
                            // the two roundings this section always had.
                            .child(Skeleton::new().h(px(12.0)).w(px(200.0)).rounded(t.radius))
                            .child(Skeleton::new().h(px(8.0)).w(px(300.0)).rounded(t.radius))
                            .child(Skeleton::new().h(px(8.0)).w(px(250.0)).rounded(t.radius))
                            .child(Skeleton::new().secondary().h(px(60.0)).rounded(t.radius_lg)),
                    )
                    .on_hover(self.hover_info(&fi, "Skeleton", &[("bg", "skeleton", t.skeleton, "gpui-component/skeleton.rs:43")], &[], &[("animation", "a 2s literal, and unlike the Accordion's or the Switch's it is not one of the theme's motion tokens (skeleton.rs, Skeleton) -- nothing to write, upstream or here")])),
            )
            // ShimmerText
            .child(section("ShimmerText (a highlight sweeping across the label)"))
            .child(
                div()
                    .id("tt-shimmer-text")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(360.0))
                            // The highlight is mixed from the text colour, so
                            // each of these shimmers in whatever colour the
                            // native theme gave its own text.
                            .child(ShimmerText::new("Reading the desktop configuration…"))
                            .child(
                                ShimmerText::new("Resolving the palette…")
                                    .id("shimmer-slow")
                                    .duration(Duration::from_secs(3))
                                    .text_color(t.muted_foreground),
                            )
                            .child(
                                ShimmerText::new("Applying to gpui…")
                                    .id("shimmer-reverse")
                                    .reverse(true)
                                    .spread(0.5),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "ShimmerText", &[("text", "foreground", t.foreground, "gpui-component/root.rs:596"), ("second line", "muted_foreground", t.muted_foreground, "showcase")], &[], &[
                            ("highlight", "the text colour mixed with background (light) or foreground (dark), at 75%/60% peak (shimmer.rs, shimmer_highlight_color)"),
                            ("reduced motion", "gpui's App::reduce_motion: the text renders once, unanimated (shimmer.rs, ShimmerText::render)"),
                            ("sweep", "2s by default, a literal rather than a motion token (shimmer.rs, ShimmerStyle); 3s and a reversed 0.5 spread here"),
                        ])),
            )
            // Empty state
            .child(section("Empty"))
            .child(
                div()
                    .id("tt-empty")
                    .child(
                        Empty::new()
                            .w(px(360.0))
                            .header(
                                EmptyHeader::new()
                                    .media(
                                        EmptyMedia::new()
                                            .with_variant(EmptyMediaVariant::Icon)
                                            // An empty state's icon is the
                                            // large one; `EmptyMedia` takes it
                                            // as a plain child, so the size
                                            // survives.
                                            .child(native_icon(
                                                cx,
                                                IconName::Inbox,
                                                geometry::icon_size_large,
                                            )),
                                    )
                                    .title(EmptyTitle::new().child("No notifications"))
                                    .description(EmptyDescription::new().child(
                                        "Anything the application reports shows up here.",
                                    )),
                            )
                            .content(
                                EmptyContent::new().child(
                                    Button::new("empty-refresh").native(cx, geometry::button).label("Refresh").outline(),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Empty", &[("border", "border", t.border, "gpui-component/empty.rs:75"), ("media bg", "muted", t.muted, "gpui-component/empty.rs:219"), ("title", "foreground", t.foreground, "gpui-component/empty.rs:77"), ("description", "muted_foreground", t.muted_foreground, "gpui-component/empty.rs:319")], &[(
                            "border-radius",
                            format!("radius_tokens().xl: {}px", t.radius_tokens().xl.as_f32()),
                        ), ("icon size", "geometry::icon_size_large: defaults.icon_sizes.large".to_string())], &[
                            ("border style", "border_dashed with no seam to change it (empty.rs, Empty)"),
                            ("media frame", "2rem square -- the platform's font, not a literal -- and settable: EmptyMedia applies the caller's refinement last (empty.rs, EmptyMedia). Nothing sizes it to the icon inside, which is defaults.icon_sizes.large: 32px, or 48px on KDE, so the icon outgrows the frame wherever 2rem is smaller"),
                        ])),
            )
            // Tags
            .child(section("Tags (7 colors + outline)"))
            .child(
                div()
                    .id("tt-tags")
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
                            .child(Tag::primary().outline().child("Primary Outline"))
                            .child(Tag::danger().outline().child("Danger Outline")),
                    )
                    .on_hover(self.hover_info(&fi, "Tag (per variant)", &[("bg (primary)", "primary", t.primary, "gpui-component/tag.rs:29"), ("bg (secondary)", "secondary", t.secondary, "gpui-component/tag.rs:30"), ("bg (danger)", "danger", t.danger, "gpui-component/tag.rs:31"), ("bg (success)", "success", t.success, "gpui-component/tag.rs:32"), ("bg (warning)", "warning", t.warning, "gpui-component/tag.rs:33"), ("bg (info)", "info", t.info, "gpui-component/tag.rs:34"), ("text (primary)", "primary_foreground", t.primary_foreground, "gpui-component/tag.rs:71"), ("text (secondary)", "secondary_foreground", t.secondary_foreground, "gpui-component/tag.rs:78"), ("text (danger)", "danger_foreground", t.danger_foreground, "gpui-component/tag.rs:85"), ("text (success)", "success_foreground", t.success_foreground, "gpui-component/tag.rs:92"), ("text (warning)", "warning_foreground", t.warning_foreground, "gpui-component/tag.rs:99"), ("text (info)", "info_foreground", t.info_foreground, "gpui-component/tag.rs:106"), ("border (primary)", "primary", t.primary, "gpui-component/tag.rs:48"), ("border (secondary)", "border", t.border, "gpui-component/tag.rs:49"), ("border (danger)", "danger", t.danger, "gpui-component/tag.rs:50"), ("border (success)", "success", t.success, "gpui-component/tag.rs:51"), ("border (warning)", "warning", t.warning, "gpui-component/tag.rs:52"), ("border (info)", "info", t.info, "gpui-component/tag.rs:53"), ("outlined text (primary)", "primary", t.primary, "gpui-component/tag.rs:69"), ("outlined text (danger)", "danger", t.danger, "gpui-component/tag.rs:83")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("padding", "px_1p5/py_0p5 and px_2p5/py_1 per Size (tag.rs, Tag::render) -- rems, so already proportional to the platform font -- and Tag applies the caller's refinement after them anyway. native-theme states no tag widget, so there is nothing to carry. Our gap"), ("outlined fill", "transparent_white(), a literal: an outlined Tag drops its variant background entirely, keeping only the border and the text (tag.rs, Tag::render)")])),
            )
            // Badges
            .child(section("Badge"))
            .child(
                div()
                    .id("tt-badge")
                    .child(
                        h_flex()
                            .gap_8()
                            .child(
                                Badge::new()
                                    .count(5)
                                    .child(Button::new("badge-1").native(cx, geometry::button).label("Messages")),
                            )
                            .child(
                                Badge::new()
                                    .count(99)
                                    .child(Button::new("badge-2").native(cx, geometry::button).label("Notifications")),
                            )
                            .child(
                                Badge::new()
                                    .dot()
                                    .child(Button::new("badge-3").native(cx, geometry::button).label("Updates")),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Badge", &[("dot fill", "red", t.red, "gpui-component/badge.rs:125")], &[], &[("text", "white() whatever the fill, not a theme field: the fill is red, which the connector writes from its danger colour, so the count reads only where that colour is dark enough for white (badge.rs, Badge::render)"), ("dot fill, overridden", "Badge::color replaces it; red is only the default"), ("size", "a count badge has none: it is as wide as its digits, at least 0.875rem, with its text at a 10px literal at every Size, and the dot is a 6px literal. px(10) / px(16) / px(24) per Size is an icon badge's, which this demo does not show (badge.rs, Badge::render)"), ("padding", "0.125rem on a count badge at every Size, on a pill no caller can reach: Badge applies its refinement to the wrapper around the badged element, and the pill is an absolute child built after it (badge.rs, Badge::render). native-theme states no badge widget either")])),
            )
            // Marker
            .child(section("Marker (3 variants, 2 loading styles)"))
            .child(
                div()
                    .id("tt-marker")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(360.0))
                            .child(
                                Marker::new()
                                    .icon(MarkerIcon::new().child(native_icon(
                                        cx,
                                        IconName::CircleCheck,
                                        geometry::icon_size_small,
                                    )))
                                    .content(MarkerContent::new().text("Theme applied")),
                            )
                            .child(
                                Marker::new()
                                    .with_variant(MarkerVariant::Separator)
                                    .content(MarkerContent::new().text("Today")),
                            )
                            .child(
                                Marker::new()
                                    .with_variant(MarkerVariant::Border)
                                    .content(MarkerContent::new().text("Unread from here")),
                            )
                            // Loading, with the spinner the marker adds for
                            // itself when no icon slot is set.
                            .child(
                                Marker::new()
                                    .id("marker-spinner")
                                    .loading(true)
                                    .content(MarkerContent::new().text("Reading the OS theme…")),
                            )
                            .child(
                                Marker::new()
                                    .id("marker-shimmer")
                                    .loading(true)
                                    .with_loading_style(MarkerLoadingStyle::Shimmer)
                                    .content(MarkerContent::new().text("Resolving the palette…")),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Marker", &[("text", "muted_foreground", t.muted_foreground, "gpui-component/marker.rs:185"), ("separator line", "border", t.border, "gpui-component/marker.rs:200"), ("bottom border", "border", t.border, "gpui-component/marker.rs:191")], &[("icon size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                            ("row gap", "gap_2, 0.5rem -- the platform's font -- and settable: Marker applies the caller's refinement last (marker.rs, Marker::render)"),
                            ("shimmer", "the loading highlight ShimmerText paints, on the content slot only (marker.rs, Marker::render MarkerChild::Content)"),
                        ])),
            )
            // Tooltip
            .child(section("Tooltip"))
            .child(
                div()
                    .id("tt-tooltip")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                Button::new("tooltip-1").native(cx, geometry::button)
                                    .label("Hover me")
                                    .tooltip("This is a tooltip"),
                            )
                            .child(
                                Button::new("tooltip-2").native(cx, geometry::button)
                                    .label("With tooltip")
                                    .tooltip("Save file (Cmd+S)"),
                            )
                            // `Button::tooltip` takes a string and builds the
                            // tooltip itself (`button/button.rs:389`), so the
                            // only way to a refined one is to build it: that
                            // is what `geometry::tooltip` documents, and the
                            // one place the platform's tooltip padding, radius
                            // and text colour reach the popup. The width is the
                            // content element's, through `Tooltip::element`:
                            // on the bubble it would clamp the bubble and not
                            // the text, which then runs out of it.
                            .child({
                                let style = native_geometry(cx, geometry::tooltip);
                                let content = native_geometry(cx, geometry::tooltip_content);
                                div()
                                    .id("tooltip-built")
                                    .child(
                                        Button::new("tooltip-3")
                                            .native(cx, geometry::button)
                                            .label("Built by the application"),
                                    )
                                    .tooltip(move |window, cx| {
                                        let content = content.clone();
                                        refined(
                                            Tooltip::element(move |_window, _cx| {
                                                refined(
                                                    div().child(
                                                        "This popup carries geometry::tooltip \
                                                         and geometry::tooltip_content: the \
                                                         platform's padding, radius, text size \
                                                         and text colour, and the max width the \
                                                         text wraps at.",
                                                    ),
                                                    content.as_ref(),
                                                )
                                            }),
                                            style.as_ref(),
                                        )
                                        .build(window, cx)
                                    })
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "Tooltip", &[("bg", "popover", t.popover, "gpui-component/tooltip.rs:114"), ("text", "popover_foreground", t.popover_foreground, "gpui-component/tooltip.rs:115")], &[("border-radius", format!("radius: {}px", t.radius.as_f32())), ("geometry", "geometry::tooltip on an application-built Tooltip: border.padding_*, corner_radius, tooltip.font — including its colour, which upstream would otherwise paint with popover_foreground (tooltip.rs, Tooltip::render: text_color then refine_style). geometry::tooltip_content on the element passed to Tooltip::element carries tooltip.max_width, which the text wraps at".to_string())], &[
                            ("delay", "500ms, and fixed for a Button's tooltip: Button::tooltip goes through Root's tooltip overlay, whose SHOW_DELAY is a module const (gpui-base/tooltip.rs, SHOW_DELAY). Only a tooltip set on an element, as the third one here is, is gpui's own and takes tooltip_show_delay (gpui-pre/elements/div.rs, tooltip_show_delay). native-theme states no hover delay, though the platforms do -- our model's gap"),
                            ("position", "a Button's sits beside its trigger and flips to stay in the window, unless Button::tooltip_placement pins a side (gpui-base/tooltip.rs, TooltipPositioner); gpui's own sits a pixel off the pointer (gpui-pre/window.rs, prepaint_tooltip)"),
                        ])),
            )
            // Notification
            .child(section("Notification (push via WindowExt)"))
            .child(
                div()
                    .id("tt-notification")
                    .child(
                        h_flex()
                            .gap_3()
                            .child(probe(
                                PROBE_NOTIFICATION,
                                Button::new("notify-info").native(cx, geometry::button)
                                    .label("Info")
                                    .on_click(cx.listener(|_this, _ev, window, cx| {
                                        window.push_notification(
                                            Notification::info("This is an info notification.")
                                                .title("Info")
                                                .autohide(true),
                                            cx,
                                        );
                                    })),
                            ))
                            .child(Button::new("notify-success").native(cx, geometry::button).label("Success").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::success("Operation completed.")
                                            .title("Success")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            ))
                            .child(Button::new("notify-warning").native(cx, geometry::button).label("Warning").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::warning("Careful with this action.")
                                            .title("Warning")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            ))
                            .child(Button::new("notify-error").native(cx, geometry::button).label("Error").on_click(
                                cx.listener(|_this, _ev, window, cx| {
                                    window.push_notification(
                                        Notification::error("Something went wrong.")
                                            .title("Error")
                                            .autohide(true),
                                        cx,
                                    );
                                }),
                            )),
                    )
                    .on_hover(self.hover_info(&fi, "Notification", &[("bg", "popover", t.popover, "gpui-component/notification.rs:424"), ("border", "border", t.border, "gpui-component/notification.rs:423"), ("info icon", "info", t.info, "gpui-component/notification.rs:42"), ("success icon", "success", t.success, "gpui-component/notification.rs:43"), ("warning icon", "warning", t.warning, "gpui-component/notification.rs:44"), ("error icon", "danger", t.danger, "gpui-component/notification.rs:45")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("animation", "module consts, 400ms in and 200ms out, not the theme's motion tokens (notification.rs, Notification)"), ("autohide", "after 5s, a literal, unless Notification::autohide(false) or an action keeps it open (notification.rs, ToastOptions)")])),
            )
    }
}
