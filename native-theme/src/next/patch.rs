//! Tier 2 of customization (spec §7): a [`ThemePatch`], the sparse changes an
//! application lays over a resolved theme (resolution step 3, spec §6).
//!
//! Three sparse mirrors of the part styles, [`PartPatch`], [`TextPatch`] and
//! [`IconPatch`], each an `Option` per property that turns into the rule
//! engine's properties through `to_props`; a sparse mirror of the colour
//! roles, [`ColorPatch`]; and per widget an entry of [`WidgetPatchMap`]: an
//! `Option` per direct metric and, for a ruled widget, the rules appended to
//! its parts (each a [`RuleSelector`] and the patch of every part it states),
//! for a static widget the unconditional patch of each part. Every one of
//! them is `#[non_exhaustive]`, with `new()` and one chained setter per field.

use std::collections::BTreeMap;
use std::sync::Arc;

use super::color::Color;
use super::error::Error;
use super::geometry::{CornerRadius, Corners, Cursor, Edges, ShadowLayer, Stroke, Transform};
use super::layout::{ArrowSide, DialogButtonOrder, TabIndicatorSide};
use super::motion::{DialogEntranceStyle, Timing, Transition};
use super::paint::{BlendMode, GradientStop, Paint, PaintLayer};
use super::query::{
    ButtonShape, ButtonVariant, ControlSize, GroupPosition, InputVariant, ListVariant, Orientation,
    Surface, TabVariant, WidgetState,
};
use super::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use super::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, Patched, RuleSelector, RuleSet, TextProp,
};
use super::style::{
    FocusIndicator, FontStyle, FontWidth, IconRenderingMode, IconStyle, PartStyle, RippleSpec,
    TextDecoration, TextStated, TextStyle,
};
use super::typography::Typography;
use super::widgets::{
    ButtonWidget, CardWidget, CheckboxWidget, ComboBoxWidget, DialogWidget, ExpanderWidget,
    InputWidget, LinkWidget, ListWidget, MenuBarWidget, MenuWidget, PopoverWidget,
    ProgressBarWidget, RadioWidget, ResolvedTheme, ScrollbarWidget, SegmentedControlWidget,
    SeparatorWidget, SidebarWidget, SliderWidget, SpinnerWidget, SplitterWidget, StatusBarWidget,
    SwitchWidget, TabWidget, ToolbarWidget, TooltipWidget,
};

/// A struct of sparse fields, its `new()` (the patch that states nothing) and
/// one chained setter per field. A field is `opt` (an `Option` the setter
/// fills), `all` (a nested patch the setter replaces), `list` (rules the
/// setter `rule` appends to) or `selector` (a rule's [`RuleSelector`], whose
/// setter and the setters of its states and axes write it).
macro_rules! sparse {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $( $(#[$fmeta:meta])* $field:ident : $kind:ident $ty:ty ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq)]
        #[non_exhaustive]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: sparse!(@type $kind $ty), )*
        }

        impl $name {
            /// The patch that states nothing.
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            $( sparse!(@setter $kind $field $ty); )*
        }
    };
    (@type opt $ty:ty) => { Option<$ty> };
    (@type all $ty:ty) => { $ty };
    (@type list $ty:ty) => { Vec<$ty> };
    (@type selector $ty:ty) => { $ty };
    (@setter opt $field:ident $ty:ty) => {
        #[doc = concat!("This patch with `", stringify!($field), "` stated.")]
        #[must_use]
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = Some(value);
            self
        }
    };
    (@setter all $field:ident $ty:ty) => {
        #[doc = concat!("This patch with `", stringify!($field), "` replaced.")]
        #[must_use]
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = value;
            self
        }
    };
    (@setter list $field:ident $ty:ty) => {
        /// This patch with `rule` appended to its rules.
        #[must_use]
        pub fn rule(mut self, rule: $ty) -> Self {
            self.$field.push(rule);
            self
        }
    };
    (@setter selector $field:ident $ty:ty) => {
        /// This rule with its selector replaced.
        #[must_use]
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = value;
            self
        }

        /// This rule, applying only where every one of `states` is set.
        #[must_use]
        pub fn states(mut self, states: WidgetState) -> Self {
            self.$field.required_states = states;
            self
        }

        /// This rule, applying only where none of `states` is set.
        #[must_use]
        pub fn not_states(mut self, states: WidgetState) -> Self {
            self.$field.forbidden_states = states;
            self
        }

        /// This rule, limited to `size`.
        #[must_use]
        pub fn size(mut self, size: ControlSize) -> Self {
            self.$field.size = Some(size);
            self
        }

        /// This rule, limited to `surface`.
        #[must_use]
        pub fn surface(mut self, surface: Surface) -> Self {
            self.$field.surface = Some(surface);
            self
        }

        /// This rule, limited to `position`.
        #[must_use]
        pub fn position(mut self, position: GroupPosition) -> Self {
            self.$field.position = Some(position);
            self
        }

        /// This rule, limited to `orientation`.
        #[must_use]
        pub fn orientation(mut self, orientation: Orientation) -> Self {
            self.$field.orientation = Some(orientation);
            self
        }
    };
}

/// The patch type of one widget's entry in [`WidgetPatchMap`]: for a ruled
/// widget, its rule type (a selector and a patch per part) and its entry (the
/// direct metrics and the rules); for a static widget, its entry (the direct
/// metrics and an unconditional patch per part). A metric of `set` is a plain
/// field of the widget, one of `opt` an `Option`; each names the check
/// [`validate`](ThemePatch::validate) runs on it.
macro_rules! widget_patch {
    (
        ruled $widget:ident : $Widget:ident, $Patch:ident, $Rule:ident<$M:ty> {
            parts { $( $part:ident : $PartPatch:ident ),* $(,)? }
            set { $( $set:ident : $set_ty:ty => $set_check:ident ),* $(,)? }
            opt { $( $opt:ident : $opt_ty:ty => $opt_check:ident ),* $(,)? }
        }
    ) => {
        sparse! {
            #[doc = concat!(
                "A rule appended to the parts of `", stringify!($widget),
                "`: where its selector matches, each part's patch applies, after the preset's rules."
            )]
            pub struct $Rule {
                /// Where the rule applies.
                selector: selector RuleSelector<$M>,
                $(
                    #[doc = concat!("The properties of `", stringify!($part), "`.")]
                    $part: all $PartPatch,
                )*
            }
        }

        sparse! {
            #[doc = concat!(
                "The patch of `", stringify!($widget),
                "`: its direct metrics and the rules appended to its parts."
            )]
            pub struct $Patch {
                $(
                    #[doc = concat!("`", stringify!($set), "`, where stated.")]
                    $set: opt $set_ty,
                )*
                $(
                    #[doc = concat!("`", stringify!($opt), "`, where stated.")]
                    $opt: opt $opt_ty,
                )*
                /// The rules, appended in order.
                rules: list $Rule,
            }
        }

        impl $Patch {
            /// The first invalid value, its path under `path`.
            fn check(&self, path: &str) -> Result<(), Error> {
                $(
                    if let Some(value) = &self.$set {
                        check::$set_check(&format!("{path}.{}", stringify!($set)), value)?;
                    }
                )*
                $(
                    if let Some(value) = &self.$opt {
                        check::$opt_check(&format!("{path}.{}", stringify!($opt)), value)?;
                    }
                )*
                for (index, rule) in self.rules.iter().enumerate() {
                    $( rule.$part.check(&format!("{path}.rules[{index}].{}", stringify!($part)))?; )*
                }
                Ok(())
            }

            /// Sets the metrics this patch states and appends its rules to
            /// each part, in order; a rule that states nothing of a part
            /// appends nothing to it.
            fn apply_to(&self, widget: &mut $Widget) {
                $( if let Some(value) = self.$set { widget.$set = value; } )*
                $( if let Some(value) = self.$opt { widget.$opt = Some(value); } )*
                $(
                    widget.$part.append(self.rules.iter().filter_map(|rule| {
                        let props: Vec<_> = rule.$part.to_props().collect();
                        (!props.is_empty()).then_some((rule.selector, props))
                    }));
                )*
            }
        }

        impl RescaledWidget for $Widget {
            fn rescale(&mut self, rescale: &Rescale) {
                $( rescale.rules(&mut self.$part); )*
            }
        }
    };
    (
        static $widget:ident : $Widget:ident, $Patch:ident {
            parts { $( $part:ident : $PartPatch:ident ),* $(,)? }
            set { $( $set:ident : $set_ty:ty => $set_check:ident ),* $(,)? }
            opt { $( $opt:ident : $opt_ty:ty => $opt_check:ident ),* $(,)? }
        }
    ) => {
        sparse! {
            #[doc = concat!(
                "The patch of `", stringify!($widget),
                "`: its direct metrics and the properties of each part, applied unconditionally."
            )]
            pub struct $Patch {
                $(
                    #[doc = concat!("`", stringify!($set), "`, where stated.")]
                    $set: opt $set_ty,
                )*
                $(
                    #[doc = concat!("`", stringify!($opt), "`, where stated.")]
                    $opt: opt $opt_ty,
                )*
                $(
                    #[doc = concat!("The properties of `", stringify!($part), "`.")]
                    $part: all $PartPatch,
                )*
            }
        }

        impl $Patch {
            /// The first invalid value, its path under `path`.
            fn check(&self, path: &str) -> Result<(), Error> {
                $(
                    if let Some(value) = &self.$set {
                        check::$set_check(&format!("{path}.{}", stringify!($set)), value)?;
                    }
                )*
                $(
                    if let Some(value) = &self.$opt {
                        check::$opt_check(&format!("{path}.{}", stringify!($opt)), value)?;
                    }
                )*
                $( self.$part.check(&format!("{path}.{}", stringify!($part)))?; )*
                Ok(())
            }

            /// Sets the metrics this patch states and applies each part's
            /// properties onto the part.
            fn apply_to(&self, widget: &mut $Widget) {
                $( if let Some(value) = self.$set { widget.$set = value; } )*
                $( if let Some(value) = self.$opt { widget.$opt = Some(value); } )*
                $(
                    for prop in self.$part.to_props() {
                        widget.$part.apply(&prop);
                    }
                )*
            }
        }

        impl RescaledWidget for $Widget {
            fn rescale(&mut self, _rescale: &Rescale) {
                $( _rescale.style(&mut self.$part); )*
            }
        }
    };
}

// ---------------------------------------------------------------------------
// The part, text and icon patches
// ---------------------------------------------------------------------------

sparse! {
    /// A sparse [`PartStyle`]: each property it
    /// states becomes a [`PartProp`].
    pub struct PartPatch {
        /// `fill`.
        fill: opt Paint,
        /// `stroke`.
        stroke: opt Stroke,
        /// `corners`.
        corners: opt Corners,
        /// `shadows`.
        shadows: opt Option<Arc<[ShadowLayer]>>,
        /// `opacity`.
        opacity: opt f32,
        /// `transform`.
        transform: opt Transform,
        /// `padding`, all four sides.
        padding: opt Edges,
        /// `margin`, all four sides.
        margin: opt Edges,
        /// `min_size`, per dimension: a `Some` replaces, a `None` keeps.
        min_size: opt [Option<f32>; 2],
        /// `max_size`, per dimension: a `Some` replaces, a `None` keeps.
        max_size: opt [Option<f32>; 2],
        /// `focus`.
        focus: opt Option<Arc<FocusIndicator>>,
        /// `ripple`.
        ripple: opt Option<Arc<RippleSpec>>,
        /// `transition`.
        transition: opt Option<Transition>,
        /// `cursor`.
        cursor: opt Cursor,
    }
}

impl PartPatch {
    /// The properties this patch states, in field order.
    pub fn to_props(&self) -> impl Iterator<Item = PartProp> {
        [
            self.fill.clone().map(PartProp::Fill),
            self.stroke.clone().map(PartProp::Stroke),
            self.corners.map(PartProp::Corners),
            self.shadows.clone().map(PartProp::Shadows),
            self.opacity.map(PartProp::Opacity),
            self.transform.map(PartProp::Transform),
            self.padding.map(PartProp::Padding),
            self.margin.map(PartProp::Margin),
            self.min_size.map(PartProp::MinSize),
            self.max_size.map(PartProp::MaxSize),
            self.focus.clone().map(PartProp::Focus),
            self.ripple.clone().map(PartProp::Ripple),
            self.transition.map(PartProp::Transition),
            self.cursor.map(PartProp::Cursor),
        ]
        .into_iter()
        .flatten()
    }

    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        if let Some(fill) = &self.fill {
            check::paint(&at("fill"), fill)?;
        }
        if let Some(stroke) = &self.stroke {
            check::stroke(&at("stroke"), stroke)?;
        }
        if let Some(corners) = &self.corners {
            check::corners(&at("corners"), corners)?;
        }
        if let Some(Some(shadows)) = &self.shadows {
            for (index, layer) in shadows.iter().enumerate() {
                check::shadow(&format!("{path}.shadows[{index}]"), layer)?;
            }
        }
        if let Some(opacity) = &self.opacity {
            check::unit(&at("opacity"), opacity)?;
        }
        if let Some(transform) = &self.transform {
            check::transform(&at("transform"), transform)?;
        }
        if let Some(padding) = &self.padding {
            check::edges(&at("padding"), padding, check::size)?;
        }
        if let Some(margin) = &self.margin {
            check::edges(&at("margin"), margin, check::finite)?;
        }
        if let Some(size) = &self.min_size {
            check::dimensions(&at("min_size"), size)?;
        }
        if let Some(size) = &self.max_size {
            check::dimensions(&at("max_size"), size)?;
        }
        if let Some(Some(focus)) = &self.focus {
            check::focus(&at("focus"), focus)?;
        }
        if let Some(Some(ripple)) = &self.ripple {
            check::color(&at("ripple.color"), &ripple.color)?;
        }
        if let Some(Some(transition)) = &self.transition {
            check::transition(&at("transition"), transition)?;
        }
        Ok(())
    }
}

sparse! {
    /// A sparse [`TextStyle`]: each property it
    /// states becomes a [`TextProp`].
    pub struct TextPatch {
        /// `family`.
        family: opt Arc<str>,
        /// `color`.
        color: opt Color,
        /// `size_px`; it also clears `defined_size`.
        size_px: opt f32,
        /// `weight`.
        weight: opt u16,
        /// `style`.
        style: opt FontStyle,
        /// `width`.
        width: opt FontWidth,
        /// `line_height`.
        line_height: opt f32,
        /// `letter_spacing`.
        letter_spacing: opt f32,
        /// `optical_size`.
        optical_size: opt Option<f32>,
        /// `decoration`.
        decoration: opt TextDecoration,
        /// `blend_mode`.
        blend_mode: opt BlendMode,
    }
}

impl TextPatch {
    /// The properties this patch states, in field order.
    pub fn to_props(&self) -> impl Iterator<Item = TextProp> {
        [
            self.family.clone().map(TextProp::Family),
            self.color.map(TextProp::Color),
            self.size_px.map(TextProp::SizePx),
            self.weight.map(TextProp::Weight),
            self.style.map(TextProp::Style),
            self.width.map(TextProp::Width),
            self.line_height.map(TextProp::LineHeight),
            self.letter_spacing.map(TextProp::LetterSpacing),
            self.optical_size.map(TextProp::OpticalSize),
            self.decoration.map(TextProp::Decoration),
            self.blend_mode.map(TextProp::BlendMode),
        ]
        .into_iter()
        .flatten()
    }

    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        if let Some(color) = &self.color {
            check::color(&at("color"), color)?;
        }
        if let Some(size) = &self.size_px {
            check::size(&at("size_px"), size)?;
        }
        if let Some(height) = &self.line_height {
            check::size(&at("line_height"), height)?;
        }
        if let Some(spacing) = &self.letter_spacing {
            check::finite(&at("letter_spacing"), spacing)?;
        }
        if let Some(Some(size)) = &self.optical_size {
            check::size(&at("optical_size"), size)?;
        }
        if let Some(decoration) = &self.decoration {
            if let Some(color) = &decoration.color {
                check::color(&at("decoration.color"), color)?;
            }
            if let Some(thickness) = &decoration.thickness {
                check::size(&at("decoration.thickness"), thickness)?;
            }
        }
        Ok(())
    }
}

sparse! {
    /// A sparse [`IconStyle`]: each property it
    /// states becomes an [`IconProp`].
    pub struct IconPatch {
        /// `size`.
        size: opt f32,
        /// `color`.
        color: opt Color,
        /// `opacity`.
        opacity: opt f32,
        /// `weight`.
        weight: opt u16,
        /// `rendering_mode`.
        rendering_mode: opt IconRenderingMode,
        /// `stroke_width`.
        stroke_width: opt Option<f32>,
        /// `blend_mode`.
        blend_mode: opt BlendMode,
    }
}

impl IconPatch {
    /// The properties this patch states, in field order.
    pub fn to_props(&self) -> impl Iterator<Item = IconProp> {
        [
            self.size.map(IconProp::Size),
            self.color.map(IconProp::Color),
            self.opacity.map(IconProp::Opacity),
            self.weight.map(IconProp::Weight),
            self.rendering_mode.map(IconProp::RenderingMode),
            self.stroke_width.map(IconProp::StrokeWidth),
            self.blend_mode.map(IconProp::BlendMode),
        ]
        .into_iter()
        .flatten()
    }

    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        if let Some(size) = &self.size {
            check::size(&at("size"), size)?;
        }
        if let Some(color) = &self.color {
            check::color(&at("color"), color)?;
        }
        if let Some(opacity) = &self.opacity {
            check::unit(&at("opacity"), opacity)?;
        }
        if let Some(Some(width)) = &self.stroke_width {
            check::size(&at("stroke_width"), width)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The colour roles' patch
// ---------------------------------------------------------------------------

sparse! {
    /// A sparse [`SurfaceColors`].
    pub struct SurfacePatch {
        /// `background`.
        background: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `foreground_muted`.
        foreground_muted: opt Color,
        /// `border`.
        border: opt Color,
        /// `divider`.
        divider: opt Color,
    }
}

sparse! {
    /// A sparse [`SelectionColors`].
    pub struct SelectionPatch {
        /// `background`.
        background: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `unfocused_background`.
        unfocused_background: opt Color,
        /// `unfocused_foreground`.
        unfocused_foreground: opt Color,
    }
}

sparse! {
    /// A sparse [`AccentColors`].
    pub struct AccentPatch {
        /// `fill`.
        fill: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `standalone`.
        standalone: opt Color,
    }
}

sparse! {
    /// A sparse [`StatusColors`]; a status colour it changes takes the
    /// origin [`Custom`](RoleOrigin::Custom).
    pub struct StatusPatch {
        /// `fill`.
        fill: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `standalone`.
        standalone: opt Color,
    }
}

sparse! {
    /// A sparse [`ColorRoles`]: each role it states
    /// replaces the resolved one. Only the role changes: widget parts already
    /// derived from it keep their colours (a [`Palette`](super::Palette)
    /// recolours coherently).
    pub struct ColorPatch {
        /// `window`.
        window: all SurfacePatch,
        /// `view`.
        view: all SurfacePatch,
        /// `header`.
        header: all SurfacePatch,
        /// `sidebar`.
        sidebar: all SurfacePatch,
        /// `card`.
        card: all SurfacePatch,
        /// `dialog`.
        dialog: all SurfacePatch,
        /// `popover`.
        popover: all SurfacePatch,
        /// `tooltip`.
        tooltip: all SurfacePatch,
        /// `osd`.
        osd: all SurfacePatch,
        /// `selection`.
        selection: all SelectionPatch,
        /// `text_selection`.
        text_selection: all SelectionPatch,
        /// `find_highlight`.
        find_highlight: opt Color,
        /// `scrim`.
        scrim: opt Color,
        /// `accent`.
        accent: all AccentPatch,
        /// `focus_ring`.
        focus_ring: opt Color,
        /// `caret`.
        caret: opt Color,
        /// `link`.
        link: opt Color,
        /// `link_visited`.
        link_visited: opt Color,
        /// `disabled_text`.
        disabled_text: opt Color,
        /// `shadow`.
        shadow: opt Color,
        /// `disabled_opacity`.
        disabled_opacity: opt f32,
        /// `error`.
        error: all StatusPatch,
        /// `warning`.
        warning: all StatusPatch,
        /// `success`.
        success: all StatusPatch,
        /// `info`.
        info: all StatusPatch,
    }
}

impl SurfacePatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("background", &self.background),
                ("foreground", &self.foreground),
                ("foreground_muted", &self.foreground_muted),
                ("border", &self.border),
                ("divider", &self.divider),
            ],
        )
    }
}

impl SelectionPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("background", &self.background),
                ("foreground", &self.foreground),
                ("unfocused_background", &self.unfocused_background),
                ("unfocused_foreground", &self.unfocused_foreground),
            ],
        )
    }
}

impl AccentPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("fill", &self.fill),
                ("foreground", &self.foreground),
                ("standalone", &self.standalone),
            ],
        )
    }
}

impl StatusPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("fill", &self.fill),
                ("foreground", &self.foreground),
                ("standalone", &self.standalone),
            ],
        )
    }
}

impl ColorPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        for (field, surface) in [
            ("window", &self.window),
            ("view", &self.view),
            ("header", &self.header),
            ("sidebar", &self.sidebar),
            ("card", &self.card),
            ("dialog", &self.dialog),
            ("popover", &self.popover),
            ("tooltip", &self.tooltip),
            ("osd", &self.osd),
        ] {
            surface.check(&at(field))?;
        }
        self.selection.check(&at("selection"))?;
        self.text_selection.check(&at("text_selection"))?;
        self.accent.check(&at("accent"))?;
        check::colors(
            path,
            [
                ("find_highlight", &self.find_highlight),
                ("scrim", &self.scrim),
                ("focus_ring", &self.focus_ring),
                ("caret", &self.caret),
                ("link", &self.link),
                ("link_visited", &self.link_visited),
                ("disabled_text", &self.disabled_text),
                ("shadow", &self.shadow),
            ],
        )?;
        if let Some(opacity) = &self.disabled_opacity {
            check::unit(&at("disabled_opacity"), opacity)?;
        }
        for (field, status) in [
            ("error", &self.error),
            ("warning", &self.warning),
            ("success", &self.success),
            ("info", &self.info),
        ] {
            status.check(&at(field))?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The widgets' patches
// ---------------------------------------------------------------------------

widget_patch! {
    ruled button: ButtonWidget, ButtonPatch, ButtonRule<ButtonMatcher> {
        parts { body: PartPatch, label: TextPatch, icon: IconPatch }
        set { icon_text_gap: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled input: InputWidget, InputPatch, InputRule<Option<InputVariant>> {
        parts { field: PartPatch, text: TextPatch, placeholder: TextPatch, helper_text: TextPatch }
        set {}
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled checkbox: CheckboxWidget, CheckboxPatch, CheckboxRule<()> {
        parts { indicator: PartPatch, checkmark: IconPatch, label: TextPatch }
        set { label_gap: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled radio: RadioWidget, RadioPatch, RadioRule<()> {
        parts { indicator: PartPatch, dot: PartPatch, label: TextPatch }
        set { label_gap: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled switch: SwitchWidget, SwitchPatch, SwitchRule<()> {
        parts { track: PartPatch, thumb: PartPatch, label: TextPatch }
        set {}
        opt {
            label_gap: f32 => size,
            show_state_glyphs: bool => none,
            disabled_opacity: f32 => unit,
        }
    }
}

widget_patch! {
    ruled slider: SliderWidget, SliderPatch, SliderRule<()> {
        parts { track: PartPatch, active_track: PartPatch, thumb: PartPatch, tick: PartPatch }
        set { tick_length: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled progress_bar: ProgressBarWidget, ProgressBarPatch, ProgressBarRule<()> {
        parts { track: PartPatch, fill: PartPatch }
        set {}
        opt { indeterminate_period_ms: u32 => none }
    }
}

widget_patch! {
    ruled scrollbar: ScrollbarWidget, ScrollbarPatch, ScrollbarRule<()> {
        parts { track: PartPatch, thumb: PartPatch }
        set { min_thumb_length: f32 => size, overlay_mode: bool => none }
        opt {}
    }
}

widget_patch! {
    ruled tab: TabWidget, TabPatch, TabRule<Option<TabVariant>> {
        parts {
            bar: PartPatch,
            tab: PartPatch,
            label: TextPatch,
            indicator: PartPatch,
            close_button: PartPatch,
            close_icon: IconPatch,
        }
        set {}
        opt {
            item_gap: f32 => size,
            indicator_side: TabIndicatorSide => none,
            active_outward_corner_radius: f32 => size,
        }
    }
}

widget_patch! {
    ruled menu: MenuWidget, MenuPatch, MenuRule<()> {
        parts {
            panel: PartPatch,
            item: PartPatch,
            item_label: TextPatch,
            shortcut: TextPatch,
            icon: IconPatch,
            submenu_arrow: IconPatch,
            separator: PartPatch,
        }
        set { icon_text_gap: f32 => size }
        opt {}
    }
}

widget_patch! {
    ruled menu_bar: MenuBarWidget, MenuBarPatch, MenuBarRule<()> {
        parts { bar: PartPatch, item: PartPatch, item_label: TextPatch }
        set {}
        opt { item_gap: f32 => size }
    }
}

widget_patch! {
    ruled list: ListWidget, ListPatch, ListRule<ListMatcher> {
        parts {
            container: PartPatch,
            row: PartPatch,
            highlight: PartPatch,
            item_label: TextPatch,
            header: PartPatch,
            header_label: TextPatch,
            header_separator: PartPatch,
        }
        set { grid_color: Color => color }
        opt {}
    }
}

widget_patch! {
    ruled sidebar: SidebarWidget, SidebarPatch, SidebarRule<()> {
        parts {
            container: PartPatch,
            item: PartPatch,
            item_label: TextPatch,
            item_icon: IconPatch,
            section_header: TextPatch,
        }
        set {}
        opt { item_gap: f32 => size }
    }
}

widget_patch! {
    ruled toolbar: ToolbarWidget, ToolbarPatch, ToolbarRule<()> {
        parts {
            bar: PartPatch,
            button: PartPatch,
            button_label: TextPatch,
            button_icon: IconPatch,
            separator: PartPatch,
        }
        set {}
        opt { item_gap: f32 => size }
    }
}

widget_patch! {
    ruled combo_box: ComboBoxWidget, ComboBoxPatch, ComboBoxRule<()> {
        parts { field: PartPatch, label: TextPatch, chevron: IconPatch }
        set {}
        opt { arrow_area_width: f32 => size, disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled segmented_control: SegmentedControlWidget, SegmentedControlPatch, SegmentedControlRule<()> {
        parts { track: PartPatch, segment: PartPatch, label: TextPatch, separator: PartPatch }
        set { separator_width: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled expander: ExpanderWidget, ExpanderPatch, ExpanderRule<()> {
        parts { container: PartPatch, header: PartPatch, header_label: TextPatch, arrow: IconPatch }
        set {}
        opt {
            arrow_side: ArrowSide => none,
            arrow_gap: f32 => size,
            content_indent: f32 => size,
            frame_enabled: bool => none,
        }
    }
}

widget_patch! {
    ruled card: CardWidget, CardPatch, CardRule<()> {
        parts { container: PartPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    ruled splitter: SplitterWidget, SplitterPatch, SplitterRule<()> {
        parts { handle: PartPatch }
        set { divider_width: f32 => size }
        opt {}
    }
}

widget_patch! {
    ruled link: LinkWidget, LinkPatch, LinkRule<()> {
        parts { container: PartPatch, label: TextPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static status_bar: StatusBarWidget, StatusBarPatch {
        parts { bar: PartPatch, label: TextPatch, separator: PartPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static popover: PopoverWidget, PopoverPatch {
        parts { panel: PartPatch, arrow: PartPatch, text: TextPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static dialog: DialogWidget, DialogPatch {
        parts { panel: PartPatch, title: TextPatch, body: TextPatch, icon: IconPatch }
        set { button_gap: f32 => size }
        opt {
            button_order: DialogButtonOrder => none,
            entrance: DialogEntranceStyle => none,
        }
    }
}

widget_patch! {
    static tooltip: TooltipWidget, TooltipPatch {
        parts { panel: PartPatch, text: TextPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static spinner: SpinnerWidget, SpinnerPatch {
        parts {}
        set {
            color: Color => color,
            diameter: f32 => size,
            min_diameter: f32 => size,
            stroke_width: f32 => size,
        }
        opt { period_ms: u32 => none }
    }
}

widget_patch! {
    static separator: SeparatorWidget, SeparatorPatch {
        parts {}
        set { color: Color => color, thickness: f32 => size }
        opt { inset: f32 => size, margin: f32 => finite }
    }
}

impl ButtonRule {
    /// This rule, limited to the buttons of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.selector.matcher.variant = Some(variant);
        self
    }

    /// This rule, limited to the buttons of `shape`.
    #[must_use]
    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.selector.matcher.shape = Some(shape);
        self
    }
}

impl InputRule {
    /// This rule, limited to the fields of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.selector.matcher = Some(variant);
        self
    }
}

impl TabRule {
    /// This rule, limited to the tab bars of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.selector.matcher = Some(variant);
        self
    }
}

impl ListRule {
    /// This rule, limited to the lists of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: ListVariant) -> Self {
        self.selector.matcher.variant = Some(variant);
        self
    }

    /// This rule, limited to the alternate rows (`true`) or the others.
    #[must_use]
    pub fn alternate(mut self, alternate: bool) -> Self {
        self.selector.matcher.alternate = Some(alternate);
        self
    }
}

sparse! {
    /// The widgets' patches, one entry per widget of a
    /// [`ResolvedTheme`].
    pub struct WidgetPatchMap {
        /// `button`.
        button: all ButtonPatch,
        /// `input`.
        input: all InputPatch,
        /// `text_area`.
        text_area: all InputPatch,
        /// `checkbox`.
        checkbox: all CheckboxPatch,
        /// `radio`.
        radio: all RadioPatch,
        /// `switch`.
        switch: all SwitchPatch,
        /// `slider`.
        slider: all SliderPatch,
        /// `progress_bar`.
        progress_bar: all ProgressBarPatch,
        /// `scrollbar`.
        scrollbar: all ScrollbarPatch,
        /// `tab`.
        tab: all TabPatch,
        /// `menu`.
        menu: all MenuPatch,
        /// `menu_bar`.
        menu_bar: all MenuBarPatch,
        /// `list`.
        list: all ListPatch,
        /// `sidebar`.
        sidebar: all SidebarPatch,
        /// `toolbar`.
        toolbar: all ToolbarPatch,
        /// `status_bar`.
        status_bar: all StatusBarPatch,
        /// `combo_box`.
        combo_box: all ComboBoxPatch,
        /// `segmented_control`.
        segmented_control: all SegmentedControlPatch,
        /// `expander`.
        expander: all ExpanderPatch,
        /// `card`.
        card: all CardPatch,
        /// `popover`.
        popover: all PopoverPatch,
        /// `dialog`.
        dialog: all DialogPatch,
        /// `tooltip`.
        tooltip: all TooltipPatch,
        /// `spinner`.
        spinner: all SpinnerPatch,
        /// `splitter`.
        splitter: all SplitterPatch,
        /// `separator`.
        separator: all SeparatorPatch,
        /// `link`.
        link: all LinkPatch,
    }
}

/// Runs the macro `$callback` on the field names of every entry of
/// [`WidgetPatchMap`].
macro_rules! for_each_widget {
    ($callback:ident) => {
        $callback!(
            button input text_area checkbox radio switch slider progress_bar scrollbar tab menu
            menu_bar list sidebar toolbar status_bar combo_box segmented_control expander card
            popover dialog tooltip spinner splitter separator link
        )
    };
}

impl WidgetPatchMap {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        macro_rules! check_each {
            ($($widget:ident)*) => {
                $( self.$widget.check(&format!("{path}.{}", stringify!($widget)))?; )*
            };
        }
        for_each_widget!(check_each);
        Ok(())
    }

    /// Each widget's patch applied onto the widget of `theme`.
    fn apply_to(&self, theme: &mut ResolvedTheme) {
        macro_rules! apply_each {
            ($($widget:ident)*) => {
                $( self.$widget.apply_to(&mut theme.$widget); )*
            };
        }
        for_each_widget!(apply_each);
    }
}

// ---------------------------------------------------------------------------
// The application (resolution step 3)
// ---------------------------------------------------------------------------

/// `value` written into `slot` where stated.
fn set<T: Copy>(slot: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *slot = value;
    }
}

/// `value` written into the optional `slot` where stated.
fn set_some<T: Copy>(slot: &mut Option<T>, value: Option<T>) {
    if value.is_some() {
        *slot = value;
    }
}

impl SurfacePatch {
    fn apply_to(&self, roles: &mut SurfaceColors) {
        set_some(&mut roles.background, self.background);
        set_some(&mut roles.foreground, self.foreground);
        set_some(&mut roles.foreground_muted, self.foreground_muted);
        set_some(&mut roles.border, self.border);
        set_some(&mut roles.divider, self.divider);
    }
}

impl SelectionPatch {
    fn apply_to(&self, roles: &mut SelectionColors) {
        set(&mut roles.background, self.background);
        set(&mut roles.foreground, self.foreground);
        set_some(&mut roles.unfocused_background, self.unfocused_background);
        set_some(&mut roles.unfocused_foreground, self.unfocused_foreground);
    }
}

impl AccentPatch {
    fn apply_to(&self, roles: &mut AccentColors) {
        set(&mut roles.fill, self.fill);
        set(&mut roles.foreground, self.foreground);
        set_some(&mut roles.standalone, self.standalone);
    }
}

impl StatusPatch {
    /// The roles it states written, the status taking the origin `Custom`
    /// where it states any.
    fn apply_to(&self, roles: &mut StatusColors) {
        if *self == Self::default() {
            return;
        }
        set(&mut roles.fill, self.fill);
        set(&mut roles.foreground, self.foreground);
        set_some(&mut roles.standalone, self.standalone);
        roles.origin = RoleOrigin::Custom;
    }
}

impl ColorPatch {
    /// The roles it states written; nothing derived from them changes.
    fn apply_to(&self, roles: &mut ColorRoles) {
        self.window.apply_to(&mut roles.window);
        self.view.apply_to(&mut roles.view);
        self.header.apply_to(&mut roles.header);
        self.sidebar.apply_to(&mut roles.sidebar);
        self.card.apply_to(&mut roles.card);
        self.dialog.apply_to(&mut roles.dialog);
        self.popover.apply_to(&mut roles.popover);
        self.tooltip.apply_to(&mut roles.tooltip);
        self.osd.apply_to(&mut roles.osd);
        self.selection.apply_to(&mut roles.selection);
        self.text_selection.apply_to(&mut roles.text_selection);
        set_some(&mut roles.find_highlight, self.find_highlight);
        set_some(&mut roles.scrim, self.scrim);
        self.accent.apply_to(&mut roles.accent);
        set(&mut roles.focus_ring, self.focus_ring);
        set(&mut roles.caret, self.caret);
        set(&mut roles.link, self.link);
        set_some(&mut roles.link_visited, self.link_visited);
        set(&mut roles.disabled_text, self.disabled_text);
        set(&mut roles.shadow, self.shadow);
        set(&mut roles.disabled_opacity, self.disabled_opacity);
        self.error.apply_to(&mut roles.error);
        self.warning.apply_to(&mut roles.warning);
        self.success.apply_to(&mut roles.success);
        self.info.apply_to(&mut roles.info);
    }
}

/// What one patch's walk over every part changes: the font families it
/// replaces, each as (the family it replaces, the new one), and the scales.
struct Rescale {
    ui: Option<(Arc<str>, Arc<str>)>,
    mono: Option<(Arc<str>, Arc<str>)>,
    corner: Option<f32>,
    spacing: Option<f32>,
}

impl Rescale {
    /// The walk of `patch` over a theme of `typography`: the UI family it
    /// replaces is the body's, the monospace one the monospace slot's; `None`
    /// where the patch states no family and no scale.
    fn new(patch: &ThemePatch, typography: &Typography) -> Option<Self> {
        let rescale = Self {
            ui: patch
                .font_family
                .clone()
                .map(|new| (typography.body.family.clone(), new)),
            mono: patch
                .monospace_family
                .clone()
                .map(|new| (typography.monospace.family.clone(), new)),
            corner: patch.corner_scale,
            spacing: patch.spacing_scale,
        };
        let changes = rescale.ui.is_some()
            || rescale.mono.is_some()
            || rescale.corner.is_some()
            || rescale.spacing.is_some();
        changes.then_some(rescale)
    }

    /// The new family of a text whose family is `family`: the UI family's
    /// replacement where it is the replaced UI family, else the monospace
    /// one's where it is the replaced monospace family.
    fn family(&self, family: &str) -> Option<&Arc<str>> {
        [&self.ui, &self.mono]
            .into_iter()
            .flatten()
            .find(|(old, _)| **old == *family)
            .map(|(_, new)| new)
    }

    fn corners(&self, corners: &mut Corners) {
        let Some(k) = self.corner else {
            return;
        };
        corners.radius = match corners.radius {
            CornerRadius::Uniform(r) => CornerRadius::Uniform(r * k),
            CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            } => CornerRadius::PerCorner {
                top_left: top_left * k,
                top_right: top_right * k,
                bottom_right: bottom_right * k,
                bottom_left: bottom_left * k,
            },
            CornerRadius::Concentric { inset, min_radius } => CornerRadius::Concentric {
                inset,
                min_radius: min_radius * k,
            },
            other => other,
        };
    }

    fn edges(&self, edges: &mut Edges) {
        let Some(k) = self.spacing else {
            return;
        };
        *edges = Edges::new(
            edges.top * k,
            edges.right * k,
            edges.bottom * k,
            edges.left * k,
        );
    }

    fn gap(&self, gap: &mut f32) {
        if let Some(k) = self.spacing {
            *gap *= k;
        }
    }

    fn optional_gap(&self, gap: &mut Option<f32>) {
        if let Some(gap) = gap {
            self.gap(gap);
        }
    }

    /// `rules` with its base and every property rescaled, where this walk
    /// changes anything of its kind.
    fn rules<T: Rescaled, M>(&self, rules: &mut RuleSet<T, T::Prop, M>) {
        if T::touched(self) {
            rules.edit(
                |base| base.rescale(self),
                |prop| T::rescale_prop(prop, self),
            );
        }
    }

    fn style<T: Rescaled>(&self, style: &mut T) {
        style.rescale(self);
    }

    /// The walk over the whole theme: the typography slots (the body and the
    /// monospace slot always take their new family), the layout's radii and
    /// gaps, the widgets' parts and `*_gap` metrics, and the window chrome.
    fn theme(&self, theme: &mut ResolvedTheme) {
        let typography = &mut theme.typography;
        for slot in [
            &mut typography.display,
            &mut typography.title1,
            &mut typography.title2,
            &mut typography.title3,
            &mut typography.title4,
            &mut typography.headline,
            &mut typography.callout,
            &mut typography.subheadline,
            &mut typography.caption,
        ]
        .into_iter()
        .flatten()
        {
            self.style(slot);
        }
        for (slot, family) in [
            (&mut typography.body, &self.ui),
            (&mut typography.monospace, &self.mono),
        ] {
            match family {
                Some((_, new)) => {
                    slot.family = new.clone();
                    slot.stated |= TextStated::FAMILY;
                }
                None => self.style(slot),
            }
        }

        let layout = &mut theme.layout;
        if let Some(k) = self.corner {
            layout.corner_radius *= k;
            layout.corner_radius_lg *= k;
        }
        for gap in [
            &mut layout.widget_gap,
            &mut layout.container_margin,
            &mut layout.window_margin,
            &mut layout.section_gap,
        ] {
            self.optional_gap(gap);
        }

        macro_rules! rescale_each {
            ($($widget:ident)*) => {
                $( theme.$widget.rescale(self); )*
            };
        }
        for_each_widget!(rescale_each);
        self.gap(&mut theme.button.icon_text_gap);
        self.gap(&mut theme.checkbox.label_gap);
        self.gap(&mut theme.radio.label_gap);
        self.optional_gap(&mut theme.switch.label_gap);
        self.optional_gap(&mut theme.tab.item_gap);
        self.gap(&mut theme.menu.icon_text_gap);
        self.optional_gap(&mut theme.menu_bar.item_gap);
        self.optional_gap(&mut theme.sidebar.item_gap);
        self.optional_gap(&mut theme.toolbar.item_gap);
        self.optional_gap(&mut theme.expander.arrow_gap);
        self.gap(&mut theme.dialog.button_gap);

        let window = &mut theme.window;
        window.edit_parts(|part| self.style(part), |text| self.style(text));
        self.rules(&mut window.caption_button);
        self.rules(&mut window.caption_icon);
    }
}

/// A style a patch's walk rescales, with the kind of property its rules
/// state.
trait Rescaled {
    /// The property kind.
    type Prop: Clone;

    /// Whether the walk changes anything of this kind.
    fn touched(rescale: &Rescale) -> bool;

    fn rescale(&mut self, rescale: &Rescale);

    fn rescale_prop(prop: &mut Self::Prop, rescale: &Rescale);
}

impl Rescaled for PartStyle {
    type Prop = PartProp;

    fn touched(rescale: &Rescale) -> bool {
        rescale.corner.is_some() || rescale.spacing.is_some()
    }

    /// The corners, the padding and the margin; never a stroke, a width or a
    /// size.
    fn rescale(&mut self, rescale: &Rescale) {
        rescale.corners(&mut self.corners);
        rescale.edges(&mut self.padding);
        rescale.edges(&mut self.margin);
    }

    fn rescale_prop(prop: &mut PartProp, rescale: &Rescale) {
        match prop {
            PartProp::Corners(corners) => rescale.corners(corners),
            PartProp::Padding(edges)
            | PartProp::PaddingSides(edges, _)
            | PartProp::Margin(edges)
            | PartProp::MarginSides(edges, _) => rescale.edges(edges),
            _ => {}
        }
    }
}

impl Rescaled for TextStyle {
    type Prop = TextProp;

    fn touched(rescale: &Rescale) -> bool {
        rescale.ui.is_some() || rescale.mono.is_some()
    }

    /// The family, where it is stated and one the walk replaces.
    fn rescale(&mut self, rescale: &Rescale) {
        if !self.stated.contains(TextStated::FAMILY) {
            return;
        }
        if let Some(new) = rescale.family(&self.family) {
            self.family = new.clone();
        }
    }

    fn rescale_prop(prop: &mut TextProp, rescale: &Rescale) {
        if let TextProp::Family(family) = prop
            && let Some(new) = rescale.family(family)
        {
            *family = new.clone();
        }
    }
}

impl Rescaled for IconStyle {
    type Prop = IconProp;

    fn touched(_rescale: &Rescale) -> bool {
        false
    }

    fn rescale(&mut self, _rescale: &Rescale) {}

    fn rescale_prop(_prop: &mut IconProp, _rescale: &Rescale) {}
}

/// A widget whose parts a patch's walk rescales.
trait RescaledWidget {
    fn rescale(&mut self, rescale: &Rescale);
}

impl ResolvedTheme {
    /// `patch` applied onto this theme (spec §6, resolution step 3), as
    /// [`Theme::with_patch`](super::Theme::with_patch) applies it; for use
    /// inside a transform. In order: the colour roles it states (only the
    /// roles: the parts derived from them keep their colours); the font
    /// families, in the typography slots and in every text part and rule
    /// whose family is the one replaced; the corner radii and the spacing
    /// (never a stroke width, `border_width`, `border_opacity` or a
    /// `min_size`); the widgets' rules, appended after the preset's, the
    /// static widgets' part patches and the direct metrics; the custom
    /// tokens, merged. A patch's scales therefore leave the rules it
    /// appends as they are written.
    pub fn apply_patch(&mut self, patch: &ThemePatch) {
        patch.color_overrides.apply_to(&mut self.colors);
        if let Some(rescale) = Rescale::new(patch, &self.typography) {
            rescale.theme(self);
        }
        patch.widget_rules.apply_to(self);
        self.custom_colors.extend(
            patch
                .custom_colors
                .iter()
                .map(|(key, color)| (key.clone(), *color)),
        );
        self.custom_metrics.extend(
            patch
                .custom_metrics
                .iter()
                .map(|(key, value)| (key.clone(), *value)),
        );
    }
}

// ---------------------------------------------------------------------------
// ThemePatch
// ---------------------------------------------------------------------------

/// Sparse changes laid over a resolved theme (spec §7, Tier 2), applied by
/// [`Theme::with_patch`](super::Theme::with_patch) after the preset's
/// builder and before the transforms, or by
/// [`ResolvedTheme::apply_patch`] inside
/// a transform. Within a patch the fields apply in the order they are
/// declared here (spec §6, resolution step 3), so a patch's scales do not
/// touch the rules it appends.
///
/// ```text
/// ThemePatch::new().button(ButtonRule::new().variant(Primary).states(HOVER).body(PartPatch::new().fill(..)))
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ThemePatch {
    /// The factor of every concrete corner radius.
    pub corner_scale: Option<f32>,
    /// The factor of the spacing: the global gaps, every part's padding and
    /// margin, and the widgets' `*_gap` metrics.
    pub spacing_scale: Option<f32>,
    /// The UI font family, replacing the resolved one wherever it stands.
    pub font_family: Option<Arc<str>>,
    /// The monospace font family, replacing the resolved one wherever it
    /// stands.
    pub monospace_family: Option<Arc<str>>,
    /// The colour roles it replaces.
    pub color_overrides: ColorPatch,
    /// Per widget: the rules appended to a ruled widget's parts, the
    /// unconditional patches of a static widget's parts, and the direct
    /// metrics.
    pub widget_rules: WidgetPatchMap,
    /// The application's own colour tokens, merged into the resolved
    /// theme's.
    pub custom_colors: BTreeMap<String, Color>,
    /// The application's own metric tokens, merged into the resolved theme's.
    pub custom_metrics: BTreeMap<String, f32>,
}

/// A scale as the setters take it: clamped to the finite values `>= 0.0`, and
/// `1.0` (no change) where it is not a number.
fn clamped_scale(scale: f32) -> f32 {
    if scale.is_nan() {
        1.0
    } else {
        scale.clamp(0.0, f32::MAX)
    }
}

/// One shortcut per ruled widget: `ThemePatch::new().button(rule)` appends
/// `rule` to the button's rules.
macro_rules! rule_shortcuts {
    ($( $widget:ident : $Rule:ident ),* $(,)?) => {
        impl ThemePatch {
            $(
                #[doc = concat!("This patch with `rule` appended to the rules of `", stringify!($widget), "`.")]
                #[must_use]
                pub fn $widget(mut self, rule: $Rule) -> Self {
                    self.widget_rules.$widget.rules.push(rule);
                    self
                }
            )*
        }
    };
}

rule_shortcuts! {
    button: ButtonRule,
    input: InputRule,
    text_area: InputRule,
    checkbox: CheckboxRule,
    radio: RadioRule,
    switch: SwitchRule,
    slider: SliderRule,
    progress_bar: ProgressBarRule,
    scrollbar: ScrollbarRule,
    tab: TabRule,
    menu: MenuRule,
    menu_bar: MenuBarRule,
    list: ListRule,
    sidebar: SidebarRule,
    toolbar: ToolbarRule,
    combo_box: ComboBoxRule,
    segmented_control: SegmentedControlRule,
    expander: ExpanderRule,
    card: CardRule,
    splitter: SplitterRule,
    link: LinkRule,
}

impl ThemePatch {
    /// The patch that states nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// This patch with the corner radii scaled by `scale`, clamped to the
    /// finite values `>= 0.0` (`1.0` where it is not a number).
    #[must_use]
    pub fn corner_scale(mut self, scale: f32) -> Self {
        self.corner_scale = Some(clamped_scale(scale));
        self
    }

    /// This patch with the spacing scaled by `scale`, clamped to the finite
    /// values `>= 0.0` (`1.0` where it is not a number).
    #[must_use]
    pub fn spacing_scale(mut self, scale: f32) -> Self {
        self.spacing_scale = Some(clamped_scale(scale));
        self
    }

    /// This patch with the UI font family `family`.
    #[must_use]
    pub fn font_family(mut self, family: impl Into<Arc<str>>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// This patch with the monospace font family `family`.
    #[must_use]
    pub fn monospace_family(mut self, family: impl Into<Arc<str>>) -> Self {
        self.monospace_family = Some(family.into());
        self
    }

    /// This patch with its colour overrides replaced.
    #[must_use]
    pub fn color_overrides(mut self, colors: ColorPatch) -> Self {
        self.color_overrides = colors;
        self
    }

    /// This patch with its widget patches replaced.
    #[must_use]
    pub fn widget_rules(mut self, widgets: WidgetPatchMap) -> Self {
        self.widget_rules = widgets;
        self
    }

    /// This patch with the custom colour `key` set to `color`.
    #[must_use]
    pub fn custom_color(mut self, key: impl Into<String>, color: Color) -> Self {
        self.custom_colors.insert(key.into(), color);
        self
    }

    /// This patch with the custom metric `key` set to `value`.
    #[must_use]
    pub fn custom_metric(mut self, key: impl Into<String>, value: f32) -> Self {
        self.custom_metrics.insert(key.into(), value);
        self
    }

    /// The strict check: every number finite, every size (a scale, a radius,
    /// a width, a blur, a padding, a gap, a font or icon size) `>= 0.0`, every
    /// opacity (a colour's alpha included) in `0.0..=1.0`. The first value
    /// that fails is returned as [`Error::InvalidTheme`], its path as a TOML
    /// file would name it (`widgets.button.rules[0].body.opacity`).
    ///
    /// A static widget's entry holds no rule, so a state selector on a static
    /// widget cannot be written into a patch.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidTheme`] for the first invalid value.
    pub fn validate(&self) -> Result<(), Error> {
        if let Some(scale) = &self.corner_scale {
            check::size("metrics.corner_scale", scale)?;
        }
        if let Some(scale) = &self.spacing_scale {
            check::size("metrics.spacing_scale", scale)?;
        }
        self.color_overrides.check("colors")?;
        self.widget_rules.check("widgets")?;
        for (key, color) in &self.custom_colors {
            check::color(&format!("custom_colors.{key}"), color)?;
        }
        for (key, value) in &self.custom_metrics {
            check::finite(&format!("custom_metrics.{key}"), value)?;
        }
        Ok(())
    }
}

/// The checks of [`ThemePatch::validate`], each returning the
/// [`Error::InvalidTheme`] of the value at `path` where it fails.
mod check {
    use super::{
        Color, CornerRadius, Corners, Edges, Error, FocusIndicator, GradientStop, Paint,
        PaintLayer, ShadowLayer, Stroke, Timing, Transform, Transition,
    };

    fn invalid(path: &str, reason: &str) -> Error {
        Error::InvalidTheme {
            path: path.to_owned(),
            reason: reason.to_owned(),
        }
    }

    /// A value without a numeric check.
    pub(super) fn none<T>(_path: &str, _value: &T) -> Result<(), Error> {
        Ok(())
    }

    /// A finite number.
    pub(super) fn finite(path: &str, value: &f32) -> Result<(), Error> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(invalid(path, "not a finite number"))
        }
    }

    /// A size: finite and `>= 0.0`.
    pub(super) fn size(path: &str, value: &f32) -> Result<(), Error> {
        finite(path, value)?;
        if *value >= 0.0 {
            Ok(())
        } else {
            Err(invalid(path, "negative"))
        }
    }

    /// An opacity: finite and in `0.0..=1.0`.
    pub(super) fn unit(path: &str, value: &f32) -> Result<(), Error> {
        finite(path, value)?;
        if (0.0..=1.0).contains(value) {
            Ok(())
        } else {
            Err(invalid(path, "outside 0.0..=1.0"))
        }
    }

    /// A colour: finite channels, its alpha an opacity.
    pub(super) fn color(path: &str, color: &Color) -> Result<(), Error> {
        finite(&format!("{path}.r"), &color.r)?;
        finite(&format!("{path}.g"), &color.g)?;
        finite(&format!("{path}.b"), &color.b)?;
        unit(&format!("{path}.a"), &color.a)
    }

    /// The colours of `fields` that are stated, each under `path`.
    pub(super) fn colors<const N: usize>(
        path: &str,
        fields: [(&str, &Option<Color>); N],
    ) -> Result<(), Error> {
        for (field, value) in fields {
            if let Some(color) = value {
                self::color(&format!("{path}.{field}"), color)?;
            }
        }
        Ok(())
    }

    pub(super) fn paint(path: &str, paint: &Paint) -> Result<(), Error> {
        match paint {
            Paint::None => Ok(()),
            Paint::Solid(color) => self::color(path, color),
            Paint::Complex(complex) => {
                for (index, layer) in complex.layers.iter().enumerate() {
                    paint_layer(&format!("{path}.layers[{index}]"), layer)?;
                }
                Ok(())
            }
        }
    }

    fn gradient_stops(path: &str, stops: &[GradientStop]) -> Result<(), Error> {
        for (index, stop) in stops.iter().enumerate() {
            finite(&format!("{path}.stops[{index}].offset"), &stop.offset)?;
            self::color(&format!("{path}.stops[{index}].color"), &stop.color)?;
        }
        Ok(())
    }

    fn paint_layer(path: &str, layer: &PaintLayer) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        match layer {
            PaintLayer::Solid { color, .. } => self::color(&at("color"), color),
            PaintLayer::LinearGradient {
                angle_deg, stops, ..
            } => {
                finite(&at("angle_deg"), angle_deg)?;
                gradient_stops(path, stops)
            }
            PaintLayer::RadialGradient {
                center,
                radius,
                stops,
                ..
            } => {
                for value in center {
                    finite(&at("center"), value)?;
                }
                size(&at("radius"), radius)?;
                gradient_stops(path, stops)
            }
            PaintLayer::Material(material) => {
                size(&at("blur_radius"), &material.blur_radius)?;
                self::color(&at("tint"), &material.tint)?;
                finite(&at("saturation"), &material.saturation)?;
                unit(&at("luminosity_opacity"), &material.luminosity_opacity)?;
                unit(&at("noise_opacity"), &material.noise_opacity)?;
                self::color(&at("fallback"), &material.fallback)
            }
        }
    }

    pub(super) fn stroke(path: &str, stroke: &Stroke) -> Result<(), Error> {
        match stroke {
            Stroke::None => Ok(()),
            Stroke::Uniform { width, color, .. } => {
                size(&format!("{path}.width"), width)?;
                self::color(&format!("{path}.color"), color)
            }
            Stroke::Complex(complex) => {
                for (side, value) in [
                    ("top", &complex.top),
                    ("right", &complex.right),
                    ("bottom", &complex.bottom),
                    ("left", &complex.left),
                ] {
                    size(&format!("{path}.{side}.width"), &value.width)?;
                    paint(&format!("{path}.{side}.paint"), &value.paint)?;
                }
                Ok(())
            }
        }
    }

    pub(super) fn corners(path: &str, corners: &Corners) -> Result<(), Error> {
        let radius = format!("{path}.radius");
        match &corners.radius {
            CornerRadius::Uniform(r) => size(&radius, r),
            CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            } => {
                for r in [top_left, top_right, bottom_right, bottom_left] {
                    size(&radius, r)?;
                }
                Ok(())
            }
            CornerRadius::Concentric { inset, min_radius } => {
                finite(&format!("{radius}.inset"), inset)?;
                size(&format!("{radius}.min_radius"), min_radius)
            }
            _ => Ok(()),
        }
    }

    pub(super) fn shadow(path: &str, layer: &ShadowLayer) -> Result<(), Error> {
        finite(&format!("{path}.offset_x"), &layer.offset_x)?;
        finite(&format!("{path}.offset_y"), &layer.offset_y)?;
        size(&format!("{path}.blur"), &layer.blur)?;
        finite(&format!("{path}.spread"), &layer.spread)?;
        self::color(&format!("{path}.color"), &layer.color)
    }

    pub(super) fn transform(path: &str, transform: &Transform) -> Result<(), Error> {
        let [sx, sy] = &transform.scale;
        let [tx, ty] = &transform.translation;
        let [ox, oy] = &transform.origin;
        for (field, value) in [
            ("scale", sx),
            ("scale", sy),
            ("rotation_deg", &transform.rotation_deg),
            ("translation", tx),
            ("translation", ty),
            ("origin", ox),
            ("origin", oy),
        ] {
            finite(&format!("{path}.{field}"), value)?;
        }
        Ok(())
    }

    /// The four sides of `edges`, each checked by `side`.
    pub(super) fn edges(
        path: &str,
        edges: &Edges,
        side: fn(&str, &f32) -> Result<(), Error>,
    ) -> Result<(), Error> {
        side(&format!("{path}.top"), &edges.top)?;
        side(&format!("{path}.right"), &edges.right)?;
        side(&format!("{path}.bottom"), &edges.bottom)?;
        side(&format!("{path}.left"), &edges.left)
    }

    /// A `[width, height]` pair, each stated dimension a size.
    pub(super) fn dimensions(path: &str, value: &[Option<f32>; 2]) -> Result<(), Error> {
        for dimension in value.iter().flatten() {
            size(path, dimension)?;
        }
        Ok(())
    }

    pub(super) fn focus(path: &str, focus: &FocusIndicator) -> Result<(), Error> {
        stroke(&format!("{path}.stroke"), &focus.stroke)?;
        stroke(&format!("{path}.inner_stroke"), &focus.inner_stroke)?;
        finite(&format!("{path}.offset"), &focus.offset)?;
        match &focus.corners {
            Some(c) => corners(&format!("{path}.corners"), c),
            None => Ok(()),
        }
    }

    pub(super) fn transition(path: &str, transition: &Transition) -> Result<(), Error> {
        let timing = format!("{path}.timing");
        match &transition.timing {
            Timing::CubicBezier(x1, y1, x2, y2) => {
                for value in [x1, y1, x2, y2] {
                    finite(&timing, value)?;
                }
                Ok(())
            }
            Timing::Spring {
                mass,
                stiffness,
                damping,
            } => {
                for value in [mass, stiffness, damping] {
                    finite(&timing, value)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests;
