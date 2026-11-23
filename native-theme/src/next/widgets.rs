//! The widgets of a resolved theme and the [`ResolvedTheme`] that holds them
//! (spec §5): the first slice's six, `button`, `checkbox`, `radio`,
//! `scrollbar`, `list` and `tooltip`, here, and the other 21 in the modules
//! below (`text_area` is an `InputWidget`).
//!
//! A ruled widget holds one rule set per part and answers a query with the
//! part's concrete style, through a getter per part and a combined
//! `parts(query)`; a static widget holds its parts as public fields. Nothing
//! outside the crate constructs a widget or a resolved theme (spec §6).

use std::collections::BTreeMap;
use std::sync::Arc;

use super::color::Color;
use super::geometry::ShadowLayer;
use super::icons::IconThemeSpec;
use super::layout::LayoutMetrics;
use super::motion::{CaretSpec, MotionTokens, UiTimings};
use super::preset::Preset;
use super::query::{ButtonQuery, ListQuery, Scheme, StyleQuery, ThemeContext};
use super::roles::ColorRoles;
use super::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, RuleBuilder, RuleSet, TextProp,
};
use super::style::{FocusIndicator, IconStyle, PartStyle, TextStyle};
use super::typography::Typography;
use super::window::WindowChrome;

/// Whether a widget is the platform's own control or one the crate
/// synthesizes for a platform that lacks it (spec §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WidgetOrigin {
    /// The platform's own control, styled from its sources.
    Native,
    /// A control the platform does not have, styled by the crate from the
    /// platform's own roles, typography and corners (its labelled estimate).
    Synthesized,
}

/// A rule set of part styles.
type PartRules<M = ()> = RuleSet<PartStyle, PartProp, M>;
/// A rule set of text styles.
type TextRules<M = ()> = RuleSet<TextStyle, TextProp, M>;
/// A rule set of icon styles.
type IconRules<M = ()> = RuleSet<IconStyle, IconProp, M>;

mod card;
mod combo_box;
mod dialog;
mod expander;
mod input;
mod link;
mod menu;
mod menu_bar;
mod popover;
mod progress_bar;
mod segmented_control;
mod separator;
mod sidebar;
mod slider;
mod spinner;
mod splitter;
mod status_bar;
mod switch;
mod tab;
mod toolbar;

pub use card::{CardParts, CardWidget};
pub use combo_box::{ComboBoxParts, ComboBoxWidget};
pub use dialog::DialogWidget;
pub use expander::{ExpanderParts, ExpanderWidget};
pub use input::{InputParts, InputWidget};
pub use link::{LinkParts, LinkWidget};
pub use menu::{MenuParts, MenuWidget};
pub use menu_bar::{MenuBarParts, MenuBarWidget};
pub use popover::PopoverWidget;
pub use progress_bar::{ProgressBarParts, ProgressBarWidget};
pub use segmented_control::{SegmentedControlParts, SegmentedControlWidget};
pub use separator::SeparatorWidget;
pub use sidebar::{SidebarParts, SidebarWidget};
pub use slider::{SliderParts, SliderWidget};
pub use spinner::SpinnerWidget;
pub use splitter::{SplitterParts, SplitterWidget};
pub use status_bar::StatusBarWidget;
pub use switch::{SwitchParts, SwitchWidget};
pub use tab::{TabParts, TabWidget};
pub use toolbar::{ToolbarParts, ToolbarWidget};

/// A part rule set that states nothing: an empty base style, no rule.
fn unstated_part<M>() -> PartRules<M> {
    RuleBuilder::new(PartStyle::default()).build()
}

/// A text rule set that states nothing.
fn unstated_text<M>() -> TextRules<M> {
    RuleBuilder::new(TextStyle::default()).build()
}

/// An icon rule set that states nothing.
fn unstated_icon<M>() -> IconRules<M> {
    RuleBuilder::new(IconStyle::default()).build()
}

// ---------------------------------------------------------------------------
// button
// ---------------------------------------------------------------------------

/// A push button.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ButtonWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between the icon and the label, in logical pixels.
    pub icon_text_gap: f32,
    /// The opacity of the whole disabled button drawn as one group; `None`
    /// where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) body: PartRules<ButtonMatcher>,
    pub(crate) label: TextRules<ButtonMatcher>,
    pub(crate) icon: IconRules<ButtonMatcher>,
}

/// The parts of a button for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ButtonParts {
    /// The button's body.
    pub body: PartStyle,
    /// Its label.
    pub label: TextStyle,
    /// Its icon.
    pub icon: IconStyle,
}

impl ButtonWidget {
    /// A button of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        body: PartRules<ButtonMatcher>,
        label: TextRules<ButtonMatcher>,
        icon: IconRules<ButtonMatcher>,
        icon_text_gap: f32,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            icon_text_gap,
            disabled_opacity,
            body,
            label,
            icon,
        }
    }

    /// Whether the button is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The body's style for `query`.
    #[must_use]
    pub fn body(&self, query: impl Into<ButtonQuery>) -> PartStyle {
        self.body.eval(&query.into())
    }

    /// The label's style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<ButtonQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// The icon's style for `query`.
    #[must_use]
    pub fn icon(&self, query: impl Into<ButtonQuery>) -> IconStyle {
        self.icon.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<ButtonQuery>) -> ButtonParts {
        let query = query.into();
        ButtonParts {
            body: self.body.eval(&query),
            label: self.label.eval(&query),
            icon: self.icon.eval(&query),
        }
    }
}

// ---------------------------------------------------------------------------
// checkbox
// ---------------------------------------------------------------------------

/// A check box.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CheckboxWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between the indicator and the label, in logical pixels.
    pub label_gap: f32,
    /// The opacity of the whole disabled check box drawn as one group;
    /// `None` where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) indicator: PartRules,
    pub(crate) checkmark: IconRules,
    pub(crate) label: TextRules,
}

/// The parts of a check box for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CheckboxParts {
    /// The box.
    pub indicator: PartStyle,
    /// The check mark.
    pub checkmark: IconStyle,
    /// The label.
    pub label: TextStyle,
}

impl CheckboxWidget {
    /// A check box of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        indicator: PartRules,
        checkmark: IconRules,
        label: TextRules,
        label_gap: f32,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            label_gap,
            disabled_opacity,
            indicator,
            checkmark,
            label,
        }
    }

    /// Whether the check box is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The box's style for `query`.
    #[must_use]
    pub fn indicator(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.indicator.eval(&query.into())
    }

    /// The check mark's style for `query`.
    #[must_use]
    pub fn checkmark(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.checkmark.eval(&query.into())
    }

    /// The label's style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> CheckboxParts {
        let query = query.into();
        CheckboxParts {
            indicator: self.indicator.eval(&query),
            checkmark: self.checkmark.eval(&query),
            label: self.label.eval(&query),
        }
    }
}

// ---------------------------------------------------------------------------
// radio
// ---------------------------------------------------------------------------

/// A radio button.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadioWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between the indicator and the label, in logical pixels.
    pub label_gap: f32,
    /// The opacity of the whole disabled radio button drawn as one group;
    /// `None` where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) indicator: PartRules,
    pub(crate) dot: PartRules,
    pub(crate) label: TextRules,
}

/// The parts of a radio button for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadioParts {
    /// The circle.
    pub indicator: PartStyle,
    /// The dot inside it.
    pub dot: PartStyle,
    /// The label.
    pub label: TextStyle,
}

impl RadioWidget {
    /// A radio button of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        indicator: PartRules,
        dot: PartRules,
        label: TextRules,
        label_gap: f32,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            label_gap,
            disabled_opacity,
            indicator,
            dot,
            label,
        }
    }

    /// Whether the radio button is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The circle's style for `query`.
    #[must_use]
    pub fn indicator(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.indicator.eval(&query.into())
    }

    /// The dot's style for `query`.
    #[must_use]
    pub fn dot(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.dot.eval(&query.into())
    }

    /// The label's style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> RadioParts {
        let query = query.into();
        RadioParts {
            indicator: self.indicator.eval(&query),
            dot: self.dot.eval(&query),
            label: self.label.eval(&query),
        }
    }
}

// ---------------------------------------------------------------------------
// scrollbar
// ---------------------------------------------------------------------------

/// A scroll bar. The groove's thickness is exact: `track.min_size` equals
/// `max_size` across the bar's axis; the thumb's likewise on `thumb`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ScrollbarWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The thumb's least length along the bar, in logical pixels.
    pub min_thumb_length: f32,
    /// Whether the bar overlays the content instead of taking layout space.
    pub overlay_mode: bool,
    pub(crate) track: PartRules,
    pub(crate) thumb: PartRules,
}

/// The parts of a scroll bar for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ScrollbarParts {
    /// The groove.
    pub track: PartStyle,
    /// The handle.
    pub thumb: PartStyle,
}

impl ScrollbarWidget {
    /// A scroll bar of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        track: PartRules,
        thumb: PartRules,
        min_thumb_length: f32,
        overlay_mode: bool,
    ) -> Self {
        Self {
            origin,
            min_thumb_length,
            overlay_mode,
            track,
            thumb,
        }
    }

    /// Whether the scroll bar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The groove's style for `query`.
    #[must_use]
    pub fn track(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.track.eval(&query.into())
    }

    /// The handle's style for `query`.
    #[must_use]
    pub fn thumb(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.thumb.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> ScrollbarParts {
        let query = query.into();
        ScrollbarParts {
            track: self.track.eval(&query),
            thumb: self.thumb.eval(&query),
        }
    }
}

// ---------------------------------------------------------------------------
// list
// ---------------------------------------------------------------------------

/// A list or table view. A row's and a header's least heights are
/// `row.min_size[1]` and `header.min_size[1]`.
///
/// `row` is the row's whole rectangle (an alternate row's stripe, the
/// height, the content's padding); `highlight` is the box that marks a
/// hovered or selected row, drawn over `row` inset by its `margin`, with its
/// `corners`. A `highlight` whose margin and corners are unstated covers the
/// row's rectangle (spec §5, "The list's `row` and `highlight`").
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ListWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The colour of a table's grid lines.
    pub grid_color: Color,
    pub(crate) container: PartRules<ListMatcher>,
    pub(crate) row: PartRules<ListMatcher>,
    pub(crate) highlight: PartRules<ListMatcher>,
    pub(crate) item_label: TextRules<ListMatcher>,
    pub(crate) header: PartRules<ListMatcher>,
    pub(crate) header_label: TextRules<ListMatcher>,
    pub(crate) header_separator: PartRules<ListMatcher>,
}

/// The parts of a list for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ListParts {
    /// The view's frame and background.
    pub container: PartStyle,
    /// A row.
    pub row: PartStyle,
    /// The box that marks a hovered or selected row.
    pub highlight: PartStyle,
    /// A row's text.
    pub item_label: TextStyle,
    /// A column header.
    pub header: PartStyle,
    /// A column header's text.
    pub header_label: TextStyle,
    /// The line between column headers.
    pub header_separator: PartStyle,
}

impl ListWidget {
    /// A list of these rule sets and metrics.
    #[allow(
        clippy::too_many_arguments,
        reason = "one rule set per part of spec §5's table, and its metric"
    )]
    pub(crate) fn new(
        origin: WidgetOrigin,
        container: PartRules<ListMatcher>,
        row: PartRules<ListMatcher>,
        highlight: PartRules<ListMatcher>,
        item_label: TextRules<ListMatcher>,
        header: PartRules<ListMatcher>,
        header_label: TextRules<ListMatcher>,
        header_separator: PartRules<ListMatcher>,
        grid_color: Color,
    ) -> Self {
        Self {
            origin,
            grid_color,
            container,
            row,
            highlight,
            item_label,
            header,
            header_label,
            header_separator,
        }
    }

    /// Whether the list is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The container's style for `query`.
    #[must_use]
    pub fn container(&self, query: impl Into<ListQuery>) -> PartStyle {
        self.container.eval(&query.into())
    }

    /// A row's style for `query`.
    #[must_use]
    pub fn row(&self, query: impl Into<ListQuery>) -> PartStyle {
        self.row.eval(&query.into())
    }

    /// The style of the box that marks a hovered or selected row for
    /// `query`.
    #[must_use]
    pub fn highlight(&self, query: impl Into<ListQuery>) -> PartStyle {
        self.highlight.eval(&query.into())
    }

    /// A row's text style for `query`.
    #[must_use]
    pub fn item_label(&self, query: impl Into<ListQuery>) -> TextStyle {
        self.item_label.eval(&query.into())
    }

    /// A column header's style for `query`.
    #[must_use]
    pub fn header(&self, query: impl Into<ListQuery>) -> PartStyle {
        self.header.eval(&query.into())
    }

    /// A column header's text style for `query`.
    #[must_use]
    pub fn header_label(&self, query: impl Into<ListQuery>) -> TextStyle {
        self.header_label.eval(&query.into())
    }

    /// The header separator's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn header_separator(&self, query: impl Into<ListQuery>) -> PartStyle {
        self.header_separator.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<ListQuery>) -> ListParts {
        let query = query.into();
        ListParts {
            container: self.container.eval(&query),
            row: self.row.eval(&query),
            highlight: self.highlight.eval(&query),
            item_label: self.item_label.eval(&query),
            header: self.header.eval(&query),
            header_label: self.header_label.eval(&query),
            header_separator: self.header_separator.eval(&query),
        }
    }
}

// ---------------------------------------------------------------------------
// tooltip
// ---------------------------------------------------------------------------

/// A tooltip. Its greatest width is `panel.max_size[0]`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TooltipWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The panel.
    pub panel: PartStyle,
    /// The text.
    pub text: TextStyle,
}

impl TooltipWidget {
    /// A tooltip of these parts.
    pub(crate) fn new(origin: WidgetOrigin, panel: PartStyle, text: TextStyle) -> Self {
        Self {
            origin,
            panel,
            text,
        }
    }

    /// Whether the tooltip is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }
}

// ---------------------------------------------------------------------------
// ResolvedTheme
// ---------------------------------------------------------------------------

/// A theme resolved for one context: its identity and context, the global
/// tokens and the widgets.
///
/// Inside the crate a builder constructs it by a struct literal; outside it
/// cannot be constructed (`#[non_exhaustive]`). It holds spec §5's 27
/// widgets.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ResolvedTheme {
    /// The theme's name.
    pub name: Arc<str>,
    /// The base preset.
    pub preset: Preset,
    /// The context it was resolved for, as applied (the scheme a palette
    /// locks, the text scale sanitized).
    pub context: ThemeContext,
    /// The semantic colour roles.
    pub colors: ColorRoles,
    /// The text styles.
    pub typography: Typography,
    /// The icon theme.
    pub icons: IconThemeSpec,
    /// The layout metrics.
    pub layout: LayoutMetrics,
    /// The interaction timings.
    pub timings: UiTimings,
    /// The motion tokens.
    pub motion_tokens: MotionTokens,
    /// The global default focus ring (0.7.2's `defaults.focus_ring_*`).
    pub focus_ring: FocusIndicator,
    /// The text caret.
    pub caret: CaretSpec,
    /// The elevation levels, `0..N`, each its shadow layers.
    pub elevations: Arc<[Arc<[ShadowLayer]>]>,
    /// The window chrome.
    pub window: WindowChrome,
    /// The push button.
    pub button: ButtonWidget,
    /// The single-line text field.
    pub input: InputWidget,
    /// The multi-line text field.
    pub text_area: InputWidget,
    /// The check box.
    pub checkbox: CheckboxWidget,
    /// The radio button.
    pub radio: RadioWidget,
    /// The on/off switch.
    pub switch: SwitchWidget,
    /// The slider.
    pub slider: SliderWidget,
    /// The progress bar.
    pub progress_bar: ProgressBarWidget,
    /// The scroll bar.
    pub scrollbar: ScrollbarWidget,
    /// The tab bar.
    pub tab: TabWidget,
    /// The popup menu.
    pub menu: MenuWidget,
    /// The menu bar.
    pub menu_bar: MenuBarWidget,
    /// The list or table view.
    pub list: ListWidget,
    /// The sidebar.
    pub sidebar: SidebarWidget,
    /// The toolbar.
    pub toolbar: ToolbarWidget,
    /// The status bar.
    pub status_bar: StatusBarWidget,
    /// The combo box.
    pub combo_box: ComboBoxWidget,
    /// The segmented control.
    pub segmented_control: SegmentedControlWidget,
    /// The expander.
    pub expander: ExpanderWidget,
    /// The card.
    pub card: CardWidget,
    /// The popover.
    pub popover: PopoverWidget,
    /// The dialog.
    pub dialog: DialogWidget,
    /// The tooltip.
    pub tooltip: TooltipWidget,
    /// The busy indicator.
    pub spinner: SpinnerWidget,
    /// The splitter.
    pub splitter: SplitterWidget,
    /// The separator line.
    pub separator: SeparatorWidget,
    /// The hyperlink.
    pub link: LinkWidget,
    /// The application's own colour tokens (empty on the bundled presets).
    pub custom_colors: BTreeMap<String, Color>,
    /// The application's own metric tokens (empty on the bundled presets).
    pub custom_metrics: BTreeMap<String, f32>,
}

impl ResolvedTheme {
    /// Whether the theme was resolved for the dark scheme.
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.scheme() == Scheme::Dark
    }

    /// The scheme the theme was resolved for.
    #[must_use]
    pub fn scheme(&self) -> Scheme {
        self.context.scheme
    }

    /// The dead rules ([`RuleSet::dead_rules`]) of every rule set of the
    /// widgets, each named `<widget>.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        let mut all = vec![
            ("button.body", self.button.body.dead_rules()),
            ("button.label", self.button.label.dead_rules()),
            ("button.icon", self.button.icon.dead_rules()),
            ("checkbox.indicator", self.checkbox.indicator.dead_rules()),
            ("checkbox.checkmark", self.checkbox.checkmark.dead_rules()),
            ("checkbox.label", self.checkbox.label.dead_rules()),
            ("radio.indicator", self.radio.indicator.dead_rules()),
            ("radio.dot", self.radio.dot.dead_rules()),
            ("radio.label", self.radio.label.dead_rules()),
            ("scrollbar.track", self.scrollbar.track.dead_rules()),
            ("scrollbar.thumb", self.scrollbar.thumb.dead_rules()),
            ("list.container", self.list.container.dead_rules()),
            ("list.row", self.list.row.dead_rules()),
            ("list.highlight", self.list.highlight.dead_rules()),
            ("list.item_label", self.list.item_label.dead_rules()),
            ("list.header", self.list.header.dead_rules()),
            ("list.header_label", self.list.header_label.dead_rules()),
            (
                "list.header_separator",
                self.list.header_separator.dead_rules(),
            ),
            (
                "window.caption_button",
                self.window.caption_button.dead_rules(),
            ),
            ("window.caption_icon", self.window.caption_icon.dead_rules()),
            ("input.field", self.input.field.dead_rules()),
            ("input.text", self.input.text.dead_rules()),
            ("input.placeholder", self.input.placeholder.dead_rules()),
            ("input.helper_text", self.input.helper_text.dead_rules()),
            ("text_area.field", self.text_area.field.dead_rules()),
            ("text_area.text", self.text_area.text.dead_rules()),
            (
                "text_area.placeholder",
                self.text_area.placeholder.dead_rules(),
            ),
            (
                "text_area.helper_text",
                self.text_area.helper_text.dead_rules(),
            ),
            ("link.container", self.link.container.dead_rules()),
            ("link.label", self.link.label.dead_rules()),
            ("slider.track", self.slider.track.dead_rules()),
            ("slider.active_track", self.slider.active_track.dead_rules()),
            ("slider.thumb", self.slider.thumb.dead_rules()),
            ("slider.tick", self.slider.tick.dead_rules()),
            ("progress_bar.track", self.progress_bar.track.dead_rules()),
            ("progress_bar.fill", self.progress_bar.fill.dead_rules()),
            ("splitter.handle", self.splitter.handle.dead_rules()),
        ];
        all.extend(self.tab.dead_rules());
        all.extend(self.menu.dead_rules());
        all.extend(self.menu_bar.dead_rules());
        all.extend(self.sidebar.dead_rules());
        all.extend(self.toolbar.dead_rules());
        all.extend(self.switch.dead_rules());
        all.extend(self.combo_box.dead_rules());
        all.extend(self.segmented_control.dead_rules());
        all.extend(self.card.dead_rules());
        all.extend(self.expander.dead_rules());
        all
    }

    /// The shadow layers of elevation `level`; empty beyond the last level.
    #[must_use]
    pub fn elevation(&self, level: usize) -> &[ShadowLayer] {
        self.elevations
            .get(level)
            .map(|layers| &**layers)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests;
