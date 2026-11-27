//! Its items move to the crate root when the 0.7.2 model is removed. So far
//! it holds the visual primitives ([`Color`](crate::next::Color),
//! [`Paint`](crate::next::Paint), [`Stroke`](crate::next::Stroke),
//! [`Corners`](crate::next::Corners) and their kin), the part styles
//! ([`PartStyle`](crate::next::PartStyle), [`TextStyle`](crate::next::TextStyle),
//! [`IconStyle`](crate::next::IconStyle)), the draw-loop query
//! ([`StyleQuery`](crate::next::StyleQuery)), the rule evaluator
//! ([`RuleSet`](crate::next::RuleSet)), the global tokens' types
//! ([`ColorRoles`](crate::next::ColorRoles),
//! [`Typography`](crate::next::Typography),
//! [`IconThemeSpec`](crate::next::IconThemeSpec),
//! [`LayoutMetrics`](crate::next::LayoutMetrics),
//! [`UiTimings`](crate::next::UiTimings),
//! [`MotionTokens`](crate::next::MotionTokens),
//! [`CaretSpec`](crate::next::CaretSpec)), what a reader carries
//! ([`SystemOverrides`](crate::next::SystemOverrides)), the presets
//! ([`Preset`](crate::next::Preset)), the first slice's widgets in a
//! [`ResolvedTheme`](crate::next::ResolvedTheme), the
//! [`Palette`](crate::next::Palette) type, and a
//! [`Theme`](crate::next::Theme) that resolves one.

/// The methods and operators of a bit set over an unsigned integer whose
/// `ALL` constant holds every defined bit: `bits`, `from_bits_truncate`,
/// `is_empty`, `contains`, `intersects`, `union`, `difference`, and `BitOr`,
/// `BitOrAssign`, `BitAnd`, `BitAndAssign` and `Not`, the last masked to `ALL`
/// so that no undefined bit is ever set.
macro_rules! bit_set {
    ($name:ident, $repr:ty) => {
        impl $name {
            /// The raw bits.
            #[must_use]
            pub const fn bits(self) -> $repr {
                self.0
            }

            /// The set of `bits`, with every bit outside `ALL` cleared.
            #[must_use]
            pub const fn from_bits_truncate(bits: $repr) -> Self {
                Self(bits & Self::ALL.0)
            }

            /// Whether no bit is set.
            #[must_use]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            /// Whether every bit of `other` is set in `self`.
            #[must_use]
            pub const fn contains(self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }

            /// Whether any bit of `other` is set in `self`.
            #[must_use]
            pub const fn intersects(self, other: Self) -> bool {
                self.0 & other.0 != 0
            }

            /// The bits set in `self` or in `other`.
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            /// The bits set in `self` and not in `other`.
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0 & !other.0)
            }
        }

        impl ::core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                self.union(rhs)
            }
        }

        impl ::core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = self.union(rhs);
            }
        }

        impl ::core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl ::core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl ::core::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self {
                Self(!self.0 & Self::ALL.0)
            }
        }
    };
}

mod color;
mod error;
mod geometry;
mod icons;
mod layout;
mod motion;
mod paint;
mod palette;
#[cfg(test)]
mod parity;
mod patch;
mod platform;
mod preset;
mod presets;
mod query;
mod reader;
mod roles;
mod rules;
#[cfg(feature = "serde")]
mod serial;
mod style;
#[cfg(feature = "toml")]
mod theme_file;
mod typography;
mod widgets;
mod window;

pub use color::Color;
pub use error::{Error, ErrorKind, Result};
pub use geometry::{
    ComplexStroke, CornerCurve, CornerRadius, Corners, Cursor, Edges, ShadowLayer, Stroke,
    StrokeAlign, StrokeSide, Transform,
};
pub use icons::{IconSizes, IconThemeSpec};
pub use layout::{ArrowSide, DialogButtonOrder, LabelSide, LayoutMetrics, TabIndicatorSide};
pub use motion::{
    CaretShape, CaretSpec, DialogEntranceStyle, MotionTokens, ThemeChangePolicy, Timing,
    Transition, UiTimings,
};
pub use paint::{
    BlendMode, ComplexPaint, GradientStop, MaterialKind, MaterialSpec, Paint, PaintLayer,
};
pub use palette::Palette;
pub use patch::{
    AccentPatch, ColorPatch, IconPatch, PartPatch, SelectionPatch, StatusPatch, SurfacePatch,
    TextPatch, ThemePatch, WidgetPatchMap,
};
pub use patch::{
    ButtonPatch, ButtonRule, CardPatch, CardRule, CheckboxPatch, CheckboxRule, ComboBoxPatch,
    ComboBoxRule, DialogPatch, ExpanderPatch, ExpanderRule, InputPatch, InputRule, LinkPatch,
    LinkRule, ListPatch, ListRule, MenuBarPatch, MenuBarRule, MenuPatch, MenuRule, PopoverPatch,
    ProgressBarPatch, ProgressBarRule, RadioPatch, RadioRule, ScrollbarPatch, ScrollbarRule,
    SegmentedControlPatch, SegmentedControlRule, SeparatorPatch, SidebarPatch, SidebarRule,
    SliderPatch, SliderRule, SpinnerPatch, SplitterPatch, SplitterRule, StatusBarPatch,
    SwitchPatch, SwitchRule, TabPatch, TabRule, ToolbarPatch, ToolbarRule, TooltipPatch,
};
pub use preset::{Preset, Theme};
pub use query::{
    ButtonKind, ButtonQuery, ButtonShape, ButtonVariant, Contrast, ControlSize, Direction,
    GroupPosition, InputQuery, InputVariant, ListKind, ListQuery, ListVariant, MotionPreference,
    Orientation, Scheme, StyleQuery, Surface, TabQuery, TabVariant, ThemeContext, WidgetState,
};
pub use reader::{
    DiagnosticEntry, KdeColorEffect, KdeColorEffects, KdeContrastEffect, KdeIntensityEffect,
    KdeStateEffects, OsColors, OsFont, OsSelectionColors, OsSurfaceOverrides, SystemOverrides,
    SystemSnapshot, WindowsAccentRamp,
};
pub use roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
pub use rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, RuleSelector, RuleSet, TextProp, VariantMatcher,
};
pub use style::{
    DecorationKind, FocusIndicator, FocusKind, FocusTrigger, FontSize, FontStyle, FontWidth,
    IconRenderingMode, IconStated, IconStyle, PartStated, PartStyle, RippleSpec, TextDecoration,
    TextStated, TextStyle,
};
pub use typography::{Antialiasing, FontHinting, SubpixelOrder, TextRendering, Typography};
pub use widgets::{
    ButtonParts, ButtonWidget, CheckboxParts, CheckboxWidget, ListParts, ListWidget, RadioParts,
    RadioWidget, ResolvedTheme, ScrollbarParts, ScrollbarWidget, TooltipWidget, WidgetOrigin,
};
pub use widgets::{
    CardParts, CardWidget, ComboBoxParts, ComboBoxWidget, DialogWidget, ExpanderParts,
    ExpanderWidget, PopoverWidget, SegmentedControlParts, SegmentedControlWidget, SwitchParts,
    SwitchWidget,
};
pub use widgets::{
    InputParts, InputWidget, LinkParts, LinkWidget, ProgressBarParts, ProgressBarWidget,
    SeparatorWidget, SliderParts, SliderWidget, SpinnerWidget, SplitterParts, SplitterWidget,
};
pub use widgets::{
    MenuBarParts, MenuBarWidget, MenuParts, MenuWidget, SidebarParts, SidebarWidget,
    StatusBarWidget, TabParts, TabWidget, ToolbarParts, ToolbarWidget,
};
pub use window::{CaptionButtonKind, CaptionLayout, EdgeFlags, WindowChrome, WindowState};

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    // Spec §2, "Size-Bound Assertions".
    #[test]
    fn size_bounds_hold() {
        assert_eq!(size_of::<Color>(), 16);
        assert_eq!(size_of::<Edges>(), 16);
        assert!(size_of::<Paint>() <= 24);
        assert!(size_of::<Stroke>() <= 32);
        assert!(size_of::<PartStyle>() <= 256);
        assert_eq!(size_of::<StyleQuery<()>>(), 12);
        assert_eq!(size_of::<StyleQuery<ButtonKind>>(), 16);
    }

    fn assert_send_sync_clone_debug<T: Clone + std::fmt::Debug + Send + Sync>() {}

    // Spec §1, "Thread safety & traits".
    #[test]
    fn visual_types_are_clone_debug_send_sync() {
        assert_send_sync_clone_debug::<Paint>();
        assert_send_sync_clone_debug::<Stroke>();
        assert_send_sync_clone_debug::<PartStyle>();
        assert_send_sync_clone_debug::<TextStyle>();
        assert_send_sync_clone_debug::<IconStyle>();
        assert_send_sync_clone_debug::<RuleSet<PartStyle, PartProp>>();
        assert_send_sync_clone_debug::<RuleSet<TextStyle, TextProp>>();
        assert_send_sync_clone_debug::<RuleSet<IconStyle, IconProp>>();
        assert_send_sync_clone_debug::<ThemeContext>();
        assert_send_sync_clone_debug::<ColorRoles>();
        assert_send_sync_clone_debug::<Typography>();
        assert_send_sync_clone_debug::<IconThemeSpec>();
        assert_send_sync_clone_debug::<LayoutMetrics>();
        assert_send_sync_clone_debug::<UiTimings>();
        assert_send_sync_clone_debug::<MotionTokens>();
        assert_send_sync_clone_debug::<CaretSpec>();
        assert_send_sync_clone_debug::<CaptionLayout>();
        assert_send_sync_clone_debug::<SystemOverrides>();
        assert_send_sync_clone_debug::<SystemSnapshot>();
    }
}
