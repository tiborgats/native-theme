//! The Icons page.

use gpui::{
    Animation, AnimationExt, AnyElement, App, Context, IntoElement, ParentElement, SharedString,
    Styled, div, prelude::*, px,
};
use gpui_component::{
    ActiveTheme, Icon, Sizable, Size, h_flex, label::Label, separator::Separator, v_flex,
};
use std::time::Duration;

use native_theme::theme::{icon_name as native_icon_name, system_icon_theme};

use crate::app::Showcase;
use crate::support::{IconSource, NativeStyled, format_font_info, is_native_icon_set, section};

impl Showcase {
    /// Build the "Animated Icons" section for the Icons tab.
    fn render_animated_icons_section(&self, cx: &App) -> impl IntoElement {
        let mut cards: Vec<AnyElement> = Vec::new();

        if self.reduced_motion {
            // Show static first frames with reduced-motion label
            for (set_name, source, anim_type) in &self.animated_static_sources {
                let label_text: SharedString =
                    format!("{} - {} (reduced motion)", set_name, anim_type).into();
                cards.push(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .p_4()
                        .demo_frame(cx)
                        .child(gpui::img(source.clone()).size(px(32.)))
                        .child(Label::new(label_text).text_xs())
                        .into_any_element(),
                );
            }
        } else {
            // Frame-based animations
            for (i, (set_name, frames)) in self.animated_frame_sources.iter().enumerate() {
                let frame_idx = self.animated_frame_indices.get(i).copied().unwrap_or(0);
                let total = frames.len();
                let duration = self.animated_frame_durations.get(i).copied().unwrap_or(83);
                if let Some(source) = frames.get(frame_idx) {
                    let label_text: SharedString =
                        format!("{} - Frames: {} ({}ms)", set_name, total, duration).into();
                    cards.push(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .p_4()
                            .demo_frame(cx)
                            .child(gpui::img(source.clone()).size(px(32.)))
                            .child(Label::new(label_text).text_xs())
                            .into_any_element(),
                    );
                }
            }

            // Transform (spin) animations -- shown with opacity pulse since gpui
            // Div lacks with_transformation (only Svg has it). In real usage,
            // callers use with_spin_animation() on an Svg element.
            for (set_name, source, duration_ms) in &self.animated_spin_sources {
                let label_text: SharedString =
                    format!("{} - Spin ({}ms)", set_name, duration_ms).into();
                let spin_id = SharedString::from(format!("spinner-{}", set_name));
                let dur = Duration::from_millis(*duration_ms as u64);
                cards.push(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .p_4()
                        .demo_frame(cx)
                        .child(
                            div()
                                .size(px(32.))
                                .child(gpui::img(source.clone()).size(px(32.)))
                                .with_animation(
                                    spin_id,
                                    Animation::new(dur).repeat(),
                                    |el: gpui::Div, delta| {
                                        // Pulse opacity 0.3..1.0 to indicate animation
                                        let opacity = 0.3 + 0.7 * (1.0 - (delta * 2.0 - 1.0).abs());
                                        el.opacity(opacity)
                                    },
                                ),
                        )
                        .child(Label::new(label_text).text_xs())
                        .into_any_element(),
                );
            }
        }

        let has_items = !cards.is_empty();

        let mut section_el = v_flex().gap_2();
        section_el = section_el.child(section("Animated Icons"));

        if self.reduced_motion {
            section_el = section_el.child(
                Label::new("(prefers-reduced-motion: showing static frames)")
                    .text_xs()
                    .text_color(cx.theme().muted_foreground),
            );
        }

        let body = if has_items {
            div().child(h_flex().gap_6().flex_wrap().children(cards))
        } else {
            div().child(
                Label::new("No animated icons available for current icon sets")
                    .text_xs()
                    .text_color(cx.theme().muted_foreground),
            )
        };
        let t = cx.theme().clone();
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        section_el = section_el.child(
            body.id("tt-animated-icons")
                .on_hover(self.hover_info(&fi, "Animated Icons", &[("card border", "border", t.border, "showcase"), ("reduced-motion note", "muted_foreground", t.muted_foreground, "showcase")], &[
                    ("animations", format!("{} frame-based, {} spin", self.animated_frame_sources.len(), self.animated_spin_sources.len())),
                    ("reduced motion", format!("{}", self.reduced_motion)),
                ], &[
                    ("frame pixels", "the icon set's own indicator artwork. A symbolic frame is rasterised with the platform's font colour; nothing else about a frame is themeable (showcase-gpui/pages/icons.rs, render_animated_icons_section)"),
                    ("frame duration", "stated by the icon set that ships the animation, not by the theme — native-theme carries no animation timing (showcase-gpui/pages/icons.rs, render_animated_icons_section)"),
                    ("reduced motion", "the showcase stops its own frame timer and shows each animation's first frame (showcase-gpui/app.rs, start_animation_timer); a spinning icon needs nothing, because gpui's with_animation honours App::reduce_motion, which apply_system_theme forwards (gpui-pre/elements/animation.rs, AnimationExt). The frame timer reads the platform directly rather than that switch"),
                    ("geometry", "none: each card takes the showcase's own frame, which reads Theme::border, Theme::radius and the platform's defaults.border.line_width (showcase-gpui/support.rs, demo_frame)"),
                ])),
        );

        section_el
    }

    // -----------------------------------------------------------------------
    // Page: Icons
    // -----------------------------------------------------------------------
    pub(crate) fn render_icons_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();

        // --- Native Theme Icons section ---
        let fallback_label = if !is_native_icon_set(&self.icon_set_name) {
            " (fallback)"
        } else {
            ""
        };
        let loaded_count = self
            .loaded_icons
            .iter()
            .filter(|(_, d, _)| d.is_some())
            .count();
        let system_count = self
            .loaded_icons
            .iter()
            .filter(|(_, _, s)| *s == IconSource::System)
            .count();
        let fallback_count = self
            .loaded_icons
            .iter()
            .filter(|(_, _, s)| *s == IconSource::Fallback)
            .count();
        let is_system_set = matches!(
            self.icon_set_name.as_str(),
            "freedesktop" | "sf-symbols" | "segoe-fluent"
        );
        let native_section_title = if is_system_set {
            let detected_theme = system_icon_theme();
            format!(
                "Native Theme Icons: {} [{}/{} loaded, {} system, {} fallback]{}",
                detected_theme,
                loaded_count,
                self.loaded_icons.len(),
                system_count,
                fallback_count,
                fallback_label,
            )
        } else {
            format!(
                "Native Theme Icons: {} [{}/{} loaded]{}",
                self.icon_set_name,
                loaded_count,
                self.loaded_icons.len(),
                fallback_label,
            )
        };

        // Use the stored IconSet for tooltip icon-name lookups
        let icon_set_enum = self.icon_set_enum;
        let icon_set_label = self.icon_set_name.clone();

        // Build icon cells for loaded native icons
        let native_icon_cells: Vec<_> = self
            .loaded_icons
            .iter()
            .enumerate()
            .map(|(i, (role, _data, source))| {
                let role_name: SharedString = format!("{:?}", role).into();
                let cell_id =
                    SharedString::from(format!("native-icon-{}-{}", self.icon_set_name, i));

                let is_gpui_builtin = self.icon_set_name == "gpui-builtin";
                let icon_element = if is_gpui_builtin {
                    if let Some(icon_name) = native_theme_gpui::icons::icon_name(*role) {
                        div().child(Icon::new(icon_name).with_size(Size::Medium))
                    } else {
                        div()
                            .w(px(20.0))
                            .h(px(20.0))
                            .bg(t.skeleton)
                            .rounded(t.radius)
                    }
                } else if let Some(img_source) =
                    self.loaded_icon_sources.get(i).and_then(|s| s.clone())
                {
                    div().child(gpui::img(img_source).w(px(20.0)).h(px(20.0)))
                } else {
                    // No icon data -- gray placeholder
                    // A missing icon leaves the placeholder the theme names
                    // for one, at the theme's own rounding.
                    div()
                        .w(px(20.0))
                        .h(px(20.0))
                        .bg(t.skeleton)
                        .rounded(t.radius)
                };

                // Build tooltip text with origin info
                let tooltip_role = format!("{:?}", role);
                let tooltip_set = icon_set_label.clone();
                let tooltip_icon_name = icon_set_enum
                    .and_then(|set| native_icon_name(*role, set))
                    .unwrap_or("(unmapped)");
                let tooltip_icon_name = tooltip_icon_name.to_string();
                let source = *source;

                div()
                    .id(cell_id)
                    .flex()
                    .flex_col()
                    .items_center()
                    .py_2()
                    .px_2()
                    .gap_1()
                    .child(icon_element)
                    .child(Label::new(role_name).text_xs())
                    .on_hover(self.set_info({
                        let mut lines = format!(
                            "Role: {}\nIcon set: {}\nIcon name: {}",
                            tooltip_role, tooltip_set, tooltip_icon_name,
                        );
                        match source {
                            IconSource::System => {
                                lines.push_str("\nOrigin: OS theme");
                            }
                            IconSource::Fallback => {
                                lines.push_str(
                                    "\nOrigin: Bundled Material fallback.\n\
                                     The OS icon theme did not provide this icon,\n\
                                     so the bundled Material SVG is used instead.",
                                );
                            }
                            IconSource::Bundled => {
                                lines.push_str("\nOrigin: Bundled with native-theme");
                            }
                            IconSource::NotFound => {
                                lines.push_str(
                                    "\nOrigin: Not found.\n\
                                     No icon is available for this role in this set\n\
                                     and no bundled fallback is configured.",
                                );
                            }
                        }
                        lines
                    }))
            })
            .collect();

        // --- gpui-component IconName gallery ---
        let gpui_icon_set_label = self.icon_set_name.clone();
        let gpui_icon_cells: Vec<_> = self
            .gpui_icons
            .iter()
            .enumerate()
            .map(|(i, (name, icon, role, _data, source))| {
                let name_s: SharedString = (*name).into();
                let cell_id = SharedString::from(format!("gpui-icon-{}", i));

                // Render from cached image sources. Bundled sets (material, lucide)
                // cover all 101 icons via by-name lookup (a set without an equivalent
                // shows NotFound) — no mixing of sets.
                let is_gpui_builtin = self.icon_set_name == "gpui-builtin";
                let icon_element = if is_gpui_builtin {
                    div().child(Icon::new(icon.clone()).with_size(Size::Medium))
                } else if let Some(img_source) =
                    self.gpui_icon_sources.get(i).and_then(|s| s.clone())
                {
                    div().child(gpui::img(img_source).w(px(20.0)).h(px(20.0)))
                } else {
                    // Gray placeholder — no fallback to a different icon set
                    // A missing icon leaves the placeholder the theme names
                    // for one, at the theme's own rounding.
                    div()
                        .w(px(20.0))
                        .h(px(20.0))
                        .bg(t.skeleton)
                        .rounded(t.radius)
                };

                let tooltip_name = name.to_string();
                let tooltip_set = gpui_icon_set_label.clone();
                let tooltip_role = role.map(|r| format!("{:?}", r));
                let source = *source;

                div()
                    .id(cell_id)
                    .flex()
                    .flex_col()
                    .items_center()
                    .py_2()
                    .px_2()
                    .gap_1()
                    .child(icon_element)
                    .child(Label::new(name_s).text_xs())
                    .on_hover(self.set_info({
                        let mut lines = format!("Icon: {}", tooltip_name);
                        if let Some(ref role_str) = tooltip_role {
                            lines.push_str(&format!(
                                "\nMapped to IconRole: {}\nIcon set: {}",
                                role_str, tooltip_set,
                            ));
                        } else {
                            lines.push_str(
                                "\nNo native-theme IconRole mapping.\n\
                                 Loaded via by-name lookup from bundled icon set.",
                            );
                        }
                        match source {
                            IconSource::System => {
                                lines.push_str("\nOrigin: OS theme");
                            }
                            IconSource::Fallback => {
                                lines.push_str(
                                    "\nOrigin: Bundled Material fallback.\n\
                                     The OS icon theme did not provide this icon,\n\
                                     so the bundled Material SVG is used instead.",
                                );
                            }
                            IconSource::Bundled => {
                                lines.push_str(&format!("\nOrigin: Bundled {} SVG", tooltip_set,));
                            }
                            IconSource::NotFound => {
                                lines.push_str(
                                    "\nOrigin: Not found in selected set.\n\
                                     No icon available for this variant in the selected set.",
                                );
                            }
                        }
                        lines
                    }))
            })
            .collect();

        let mapped_count = self
            .gpui_icons
            .iter()
            .filter(|(_, _, r, _, _)| r.is_some())
            .count();

        v_flex()
            .gap_3()
            .p_4()
            // Animated Icons section
            .child(self.render_animated_icons_section(cx))
            .child(Separator::horizontal())
            // Native Theme Icons section
            .child(section(native_section_title))
            .child(
                div()
                    .id("tt-native-icons")
                    .child(div().flex().flex_wrap().gap_2().children(native_icon_cells))
                    .on_hover(self.hover_info(&fi, "Native Theme Icons", &[("missing-icon placeholder", "skeleton", t.skeleton, "showcase")], &[
                        ("icon set", self.icon_set_name.clone()),
                        ("placeholder radius", format!("radius: {}px", t.radius.as_f32())),
                    ], &[
                        ("icon pixels", "the icon theme's own files. A symbolic SVG is rasterised with the platform's font colour, which the showcase passes to the loader; a raster or multi-colour icon keeps the colours it ships with (showcase-gpui/support.rs, load_all_icons)"),
                        ("fallback", "a system icon set that has no icon for a role resolves to the bundled Material SVG instead, which the showcase detects by comparing the bytes it got back with Material's own and counts in the heading above (showcase-gpui/support.rs, load_all_icons)"),
                        ("per-icon origin", "each cell carries its own hover — the role, the icon set, the icon name the set uses, and which of the four origins it came from (showcase-gpui/pages/icons.rs, render_icons_page)"),
                        ("geometry", "none: no geometry:: builder applies to an icon grid. The cell padding and the gap are the showcase's own layout"),
                    ])),
            )
            .child(Separator::horizontal())
            // gpui-component IconName gallery
            .child(section(format!(
                "gpui-component Icons ({} variants, {} mapped to {})",
                self.gpui_icons.len(),
                mapped_count,
                self.icon_set_name,
            )))
            .child(
                div()
                    .id("tt-icons-grid")
                    .child(div().flex().flex_wrap().gap_2().children(gpui_icon_cells))
                    .on_hover(self.hover_info(
                        &fi,
                        "Icon",
                        &[],
                        &[],
                        &[
                            (
                                "color",
                                "the inherited text colour unless text_color() sets one (icon.rs, Icon::into_svg)",
                            ),
                            ("SVG shapes", "101 in gpui_component::IconName, a compatibility subset: gpui_kit_assets::IconName carries the whole Lucide catalogue (icon.rs, component_icon_names)"),
                        ],
                    )),
            )
    }
}
