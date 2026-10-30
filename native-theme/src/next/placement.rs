//! Placement (S13): where a popup, a menu, a tooltip or a sheet goes against
//! its anchor, as a widget value names it.
//!
//! These are the data types; each strategy's `place()`, from its platform's
//! source, comes with the placement unit (M3), until which a widget's
//! `Placement` values are deferred (`scripts/widget_units.toml`).

use native_theme_derive::ThemeParts;

use super::selector::Direction;
use super::units::{Axes, Point, Px, Rect, Sides, Size};
use super::value::Value;

/// A placement: a platform's strategy and its parameters; a widget value
/// (S1).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Placement {
    /// The strategy that places the element.
    pub strategy: PlacementStrategy,
    /// The parameters the strategy reads.
    #[parts(block = "fields")]
    pub params: PlacementParams,
}

/// A platform's placement strategy, one per placement row of the
/// traceability (S13), each named after the toolkit and the class or
/// component whose placement it reproduces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlacementStrategy {
    /// GTK's popover.
    GtkPopover,
    /// GTK's tooltip.
    GtkTooltip,
    /// Qt's menu.
    QtMenu,
    /// Qt's tooltip.
    QtTooltip,
    /// Qt Quick's tooltip.
    QtQuickTooltip,
    /// KDE Frameworks' `KToolTipWidget`.
    KToolTipWidget,
    /// Qt Quick's menu.
    QtQuickMenu,
    /// Qt's combo box popup.
    QtComboBox,
    /// Qt's dialog.
    QtDialog,
    /// Qt's completion box.
    QtCompletionBox,
    /// Qt's What's This help.
    QtWhatsThis,
    /// KTextEditor's completion list.
    KTextEditorCompletion,
    /// Kirigami's text actions.
    KirigamiTextActions,
    /// WinUI's flyout.
    WinUiFlyout,
    /// WinUI's full flyout.
    WinUiFlyoutFull,
    /// WinUI's tooltip.
    WinUiToolTip,
    /// WinUI's combo box popup.
    WinUiComboBox,
    /// WinUI's auto-suggest box popup.
    WinUiAutoSuggest,
    /// WinUI's teaching tip.
    WinUiTeachingTip,
    /// AppKit's menu.
    AppKitMenu,
    /// AppKit's popover.
    AppKitPopover,
    /// AppKit's tooltip.
    AppKitToolTip,
    /// AppKit's expansion tooltip.
    AppKitExpansionTip,
    /// AppKit's combo box list.
    AppKitComboList,
    /// AppKit's sheet.
    AppKitSheet,
    /// AppKit's window centring.
    AppKitWindowCenter,
    /// Material's menu.
    MaterialMenu,
    /// Material's tooltip.
    MaterialTooltip,
    /// A slot of Material's scaffold.
    MaterialScaffoldSlot(ScaffoldSlot),
    /// Material's levitated pane.
    MaterialLevitatedPane,
    /// Android's text toolbar.
    AndroidTextToolbar,
    /// UIKit's popover.
    UiKitPopover,
    /// UIKit's menu.
    UiKitMenu,
    /// UIKit's alert.
    UiKitAlert,
    /// UIKit's edit menu.
    UiKitEditMenu,
    /// KDE's command bar.
    KCommandBar,
    /// AppKit's context help.
    AppKitContextHelp,
    /// WinUI's key tip.
    WinUiKeyTip,
    /// UIKit's context menu.
    UiKitContextMenu,
    /// kirigami-addons' search popup.
    KirigamiAddonsSearchPopup,
    /// kirigami-addons' onboarding tip.
    KirigamiAddonsOnboardingTip,
    /// Centred in its host area.
    CenteredInHost,
    /// At the bottom centre of an overlay: AdwToastOverlay's and Kirigami's
    /// toasts.
    OverlayBottomCenter,
    /// A bottom sheet.
    BottomSheet,
    /// Moved clear of the on-screen keyboard, as a platform does.
    KeyboardAvoid(KeyboardAvoidance),
}

/// A slot of Material's scaffold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScaffoldSlot {
    /// The floating action button's.
    Fab,
    /// The snackbar's.
    Snackbar,
}

/// How a platform moves content clear of the on-screen keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardAvoidance {
    /// WinUI's content dialog.
    WinUiContentDialog,
    /// UIKit's alert.
    UiKitAlert,
    /// UIKit's sheet.
    UiKitSheet,
    /// WinUI's scrolling the focused element into view.
    WinUiScrollIntoView,
    /// Android's pan mode.
    AndroidPan,
    /// Android's resize mode.
    AndroidResize,
    /// Android's window insets animation.
    AndroidInsetsAnimation,
}

/// The parameters a placement strategy reads. Its side, its alignment and
/// its `Sides` are returned physical, mirrored for right-to-left (S2.4).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct PlacementParams {
    /// The gap between the anchor and the element.
    pub gap: Value<Px>,
    /// The element's least distance from the edges it is kept inside, per
    /// side.
    pub edge_margin: Sides<Value<Px>>,
    /// The anchor's side the element goes to.
    pub side: PlacementSide,
    /// The element's alignment along that side.
    pub align: PlacementAlign,
    /// Whether the element flips to the opposite side where it does not fit,
    /// per axis.
    pub flip: Axes<bool>,
    /// Whether the element slides along its side to fit, per axis.
    pub slide: Axes<bool>,
    /// The element's width against the anchor's.
    pub width: AnchorWidth,
    /// Whether the element is placed so its selected item lies over the
    /// anchor.
    pub over_selected_item: bool,
    /// The most items the element shows at once; `None`: no limit.
    pub max_visible_items: Option<u32>,
    /// Whether the element may take the screen's full height.
    pub use_full_screen_height: bool,
    /// How far the anchor's box is grown, per side, before the element is
    /// placed against it.
    pub anchor_grow: Sides<Value<Px>>,
    /// The area the element is placed in.
    pub host: HostArea,
    /// The element's least distance from the window's edges.
    pub window_edge_margin: Value<Px>,
    /// The menu drop alignment, which the preset's rule sets under
    /// `system_setting:menu_drop_alignment` (S4.2).
    pub handedness: bool,
}

/// The anchor's side an element goes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlacementSide {
    /// Below the anchor.
    Below,
    /// Above the anchor.
    Above,
    /// At the anchor's start side.
    Start,
    /// At the anchor's end side.
    End,
    /// At the pointer.
    AtPointer,
}

/// An element's alignment along its side of the anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlacementAlign {
    /// Aligned with the anchor's start.
    Start,
    /// Centred on the anchor.
    Center,
    /// Aligned with the anchor's end.
    End,
}

/// An element's width against its anchor's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnchorWidth {
    /// The element's content's width.
    Content,
    /// The content's width, and at least the anchor's.
    AtLeastAnchor,
    /// The anchor's width.
    Anchor,
}

/// The area an element is placed in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HostArea {
    /// The window.
    Window,
    /// The window's client area.
    ClientArea,
    /// The overlay the element is presented in.
    Overlay,
}

/// What a connector passes a strategy: the boxes it places against, which it
/// fills in full (S1).
#[derive(Clone, Debug, PartialEq)]
pub struct PlacementInput {
    /// The anchor's box.
    pub anchor: Rect,
    /// The element's size.
    pub size: Size,
    /// The window's box.
    pub window: Rect,
    /// The screen's work area.
    pub work_area: Rect,
    /// The pointer; `None`: none, so the anchor's start corner stands for it.
    pub pointer: Option<Point>,
    /// The selected item's box; `None`: none, so the first item stands for
    /// it.
    pub selected_item: Option<Rect>,
    /// The on-screen keyboard's box; `None`: no keyboard shown.
    pub keyboard: Option<Rect>,
    /// The element's direction, for what a strategy's source reads of it
    /// beyond the parameters, which are physical already.
    pub direction: Direction,
}

#[cfg(test)]
mod tests {
    //! Every number here is synthetic.

    use super::*;
    use crate::next::blocks::{Merge, Table};
    use crate::next::registry::{self, leaves};

    #[test]
    fn placement_resolves_s13s_params() {
        let resolved = ResolvedPlacementParams {
            gap: Some(Px(1.0)),
            edge_margin: Sides {
                top: None,
                right: None,
                bottom: None,
                left: None,
            },
            side: PlacementSide::Below,
            align: PlacementAlign::Start,
            flip: Axes {
                horizontal: true,
                vertical: false,
            },
            slide: Axes {
                horizontal: false,
                vertical: true,
            },
            width: AnchorWidth::AtLeastAnchor,
            over_selected_item: false,
            max_visible_items: None,
            use_full_screen_height: false,
            anchor_grow: Sides {
                top: Some(Px(0.0)),
                right: Some(Px(0.0)),
                bottom: Some(Px(0.0)),
                left: Some(Px(0.0)),
            },
            host: HostArea::Window,
            window_edge_margin: None,
            handedness: false,
        };
        let placement = ResolvedPlacement {
            strategy: PlacementStrategy::KeyboardAvoid(KeyboardAvoidance::AndroidPan),
            params: resolved,
        };
        assert_eq!(placement.params.gap, Some(Px(1.0)));

        let mut paths = Vec::new();
        leaves("", Placement::FIELDS, &mut paths);
        for path in [
            "strategy",
            "params.gap",
            "params.edge_margin.left",
            "params.flip.vertical",
            "params.max_visible_items",
            "params.handedness",
        ] {
            assert!(paths.iter().any(|p| p == path), "{path}");
            assert!(registry::find(Placement::FIELDS, path).is_some(), "{path}");
        }
    }

    #[test]
    fn placement_params_patch_field_by_field() {
        let mut earlier = PlacementPatch {
            params: Some(PlacementParamsPatch {
                gap: Some(Value::Stated(Px(2.0))),
                ..PlacementParamsPatch::default()
            }),
            ..PlacementPatch::default()
        };
        let later = PlacementPatch {
            strategy: Some(PlacementStrategy::MaterialScaffoldSlot(ScaffoldSlot::Fab)),
            params: Some(PlacementParamsPatch {
                side: Some(PlacementSide::Above),
                ..PlacementParamsPatch::default()
            }),
        };
        assert_eq!(earlier.merge(later), Ok(()));
        assert_eq!(
            earlier,
            PlacementPatch {
                strategy: Some(PlacementStrategy::MaterialScaffoldSlot(ScaffoldSlot::Fab)),
                params: Some(PlacementParamsPatch {
                    gap: Some(Value::Stated(Px(2.0))),
                    side: Some(PlacementSide::Above),
                    ..PlacementParamsPatch::default()
                }),
            }
        );
    }
}
