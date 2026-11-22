//! The two query levels: the resolve-time [`ThemeContext`] and the draw-loop
//! [`StyleQuery`] with its widget state and variant axes.

/// The scope a theme is resolved for: scheme, contrast, motion and the
/// accessibility settings.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ThemeContext {
    /// Light or dark.
    pub scheme: Scheme,
    /// The contrast level.
    pub contrast: Contrast,
    /// The motion preference.
    pub motion: MotionPreference,
    /// The reduce-transparency setting.
    pub reduced_transparency: bool,
    /// The bold-text setting.
    pub bold_text: bool,
    /// The button-shapes setting.
    pub button_shapes: bool,
    /// The differentiate-without-colour setting.
    pub differentiate_without_color: bool,
    /// Whether the window is active.
    pub window_active: bool,
    /// The text scale factor, `0.5..=4.0`.
    pub text_scale: f32, // default 1.0; clamped to 0.5..=4.0
    /// The reading direction.
    pub direction: Direction,
}

/// The smallest text scale (spec §3, `ThemeContext::text_scale`).
pub(crate) const TEXT_SCALE_MIN: f32 = 0.5;
/// The largest text scale (spec §3, `ThemeContext::text_scale`).
pub(crate) const TEXT_SCALE_MAX: f32 = 4.0;
/// The default text scale (spec §3, `ThemeContext::text_scale`).
pub(crate) const TEXT_SCALE_DEFAULT: f32 = 1.0;

/// `scale` clamped to `0.5..=4.0`, or the default 1.0 where it is not finite
/// (spec §3, `ThemeContext::text_scale`; §6, resolution step 1).
pub(crate) fn sanitized_text_scale(scale: f32) -> f32 {
    if scale.is_finite() {
        scale.clamp(TEXT_SCALE_MIN, TEXT_SCALE_MAX)
    } else {
        TEXT_SCALE_DEFAULT
    }
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            scheme: Scheme::Light,
            contrast: Contrast::Normal,
            motion: MotionPreference::Full,
            reduced_transparency: false,
            bold_text: false,
            button_shapes: false,
            differentiate_without_color: false,
            window_active: true,
            text_scale: TEXT_SCALE_DEFAULT,
            direction: Direction::Ltr,
        }
    }
}

impl ThemeContext {
    /// The default context: light, normal contrast, full motion, active
    /// window, text scale 1.0, left to right.
    #[must_use]
    pub fn light() -> Self {
        Self::default()
    }

    /// The default context with the dark scheme.
    #[must_use]
    pub fn dark() -> Self {
        Self::default().with_scheme(Scheme::Dark)
    }

    /// This context with `scheme`.
    #[must_use]
    pub fn with_scheme(mut self, scheme: Scheme) -> Self {
        self.scheme = scheme;
        self
    }

    /// This context with `contrast`.
    #[must_use]
    pub fn with_contrast(mut self, contrast: Contrast) -> Self {
        self.contrast = contrast;
        self
    }

    /// This context with `motion`.
    #[must_use]
    pub fn with_motion(mut self, motion: MotionPreference) -> Self {
        self.motion = motion;
        self
    }

    /// This context with the reduce-transparency setting.
    #[must_use]
    pub fn with_reduced_transparency(mut self, on: bool) -> Self {
        self.reduced_transparency = on;
        self
    }

    /// This context with the bold-text setting.
    #[must_use]
    pub fn with_bold_text(mut self, on: bool) -> Self {
        self.bold_text = on;
        self
    }

    /// This context with the button-shapes setting.
    #[must_use]
    pub fn with_button_shapes(mut self, on: bool) -> Self {
        self.button_shapes = on;
        self
    }

    /// This context with the differentiate-without-colour setting.
    #[must_use]
    pub fn with_differentiate_without_color(mut self, on: bool) -> Self {
        self.differentiate_without_color = on;
        self
    }

    /// This context with the window's active state.
    #[must_use]
    pub fn with_window_active(mut self, active: bool) -> Self {
        self.window_active = active;
        self
    }

    /// This context with the text scale `scale`, clamped to `0.5..=4.0`; a
    /// value that is not finite gives the default, 1.0.
    #[must_use]
    pub fn with_text_scale(mut self, scale: f32) -> Self {
        self.text_scale = sanitized_text_scale(scale);
        self
    }

    /// This context with `direction`.
    #[must_use]
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

/// Light or dark.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Scheme {
    /// Light.
    #[default]
    Light,
    /// Dark.
    Dark,
}

/// A contrast level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Contrast {
    /// The platform's normal contrast.
    #[default]
    Normal,
    /// Increased contrast.
    Increased,
    /// High contrast.
    High,
}

/// A motion preference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MotionPreference {
    /// Full motion.
    #[default]
    Full,
    /// Reduced motion.
    Reduced,
    /// No motion.
    None,
}

/// A reading direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Left to right.
    #[default]
    Ltr,
    /// Right to left.
    Rtl,
}

/// Zero-dependency `u32` bitflags for per-widget interaction and semantic state.
///
/// `FOCUS_VISIBLE` implies `FOCUSED`: rule evaluation adds `FOCUSED` to a
/// query that sets `FOCUS_VISIBLE`, so a rule on `FOCUSED` matches both.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WidgetState(u32);

impl WidgetState {
    /// No state.
    pub const NONE: Self = Self(0);
    /// The pointer is over the part.
    pub const HOVER: Self = Self(1 << 0);
    /// The part is pressed.
    pub const PRESSED: Self = Self(1 << 1);
    /// The part has the focus.
    pub const FOCUSED: Self = Self(1 << 2);
    /// A descendant of the part has the focus.
    pub const FOCUS_WITHIN: Self = Self(1 << 3);
    /// The part has the focus and the platform shows it (keyboard focus).
    pub const FOCUS_VISIBLE: Self = Self(1 << 4);
    /// The view holding the part (an item view, a text field) lacks the
    /// keyboard focus; every view of an inactive window is such a view.
    pub const UNFOCUSED_VIEW: Self = Self(1 << 5);
    /// The part is disabled.
    pub const DISABLED: Self = Self(1 << 6);
    /// The part is checked.
    pub const CHECKED: Self = Self(1 << 7);
    /// The part is selected.
    pub const SELECTED: Self = Self(1 << 8);
    /// The part is in its indeterminate state.
    pub const INDETERMINATE: Self = Self(1 << 9);
    /// The part is open (a menu, a combo box's popup).
    pub const OPEN: Self = Self(1 << 10);
    /// The part is its group's default (a dialog's default button).
    pub const DEFAULT: Self = Self(1 << 11);
    /// The part's value is invalid.
    pub const INVALID: Self = Self(1 << 12);
    /// The part is read-only.
    pub const READ_ONLY: Self = Self(1 << 13);
    /// The part (a link) has been visited.
    pub const VISITED: Self = Self(1 << 14);
    /// The part is being dragged.
    pub const DRAGGED: Self = Self(1 << 15);
    /// The part is a drop target under a drag.
    pub const DROP_TARGET: Self = Self(1 << 16);
    /// The part's content is scrolled.
    pub const SCROLLED: Self = Self(1 << 17);
    /// The pointer is over the part's container, not over the part (e.g. the scroll bar, not its handle).
    pub const HOVER_WITHIN: Self = Self(1 << 18);

    /// Every state.
    pub const ALL: Self = Self((1 << 19) - 1);
}

bit_set!(WidgetState, u32);

/// The size of a control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ControlSize {
    /// Mini.
    Mini,
    /// Small.
    Small,
    /// Regular.
    #[default]
    Regular,
    /// Large.
    Large,
    /// Extra large.
    ExtraLarge,
}

/// The surface a part is drawn on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Surface {
    /// A window's background.
    #[default]
    Window,
    /// A content view.
    View,
    /// A header bar.
    HeaderBar,
    /// A sidebar.
    Sidebar,
    /// A card.
    Card,
    /// A dialog.
    Dialog,
    /// A popover.
    Popover,
    /// A tooltip.
    Tooltip,
    /// An on-screen display.
    Osd,
}

/// A part's position in a group of joined parts, in reading order.
///
/// Under [`Direction::Rtl`] the resolved corners and strokes of `First` are
/// the right-hand ones.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GroupPosition {
    /// Not in a group.
    #[default]
    Single,
    /// The first.
    First,
    /// Between the first and the last.
    Middle,
    /// The last.
    Last,
}

/// An orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Orientation {
    /// Horizontal.
    #[default]
    Horizontal,
    /// Vertical.
    Vertical,
}

/// Single generic draw-loop query (`Copy`, 8 B for `V = ()`, 12 B for `ButtonQuery`/`ListQuery`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct StyleQuery<V = ()> {
    /// The widget's state.
    pub state: WidgetState,
    /// The control size.
    pub size: ControlSize,
    /// The surface it is drawn on.
    pub surface: Surface,
    /// Its position in a group.
    pub position: GroupPosition,
    /// Its orientation.
    pub orientation: Orientation,
    /// The widget's variant.
    pub variant: V,
}

impl<V: Default> StyleQuery<V> {
    /// A query of `state`, every other field at its default.
    #[must_use]
    pub fn new(state: WidgetState) -> Self {
        Self {
            state,
            size: ControlSize::default(),
            surface: Surface::default(),
            position: GroupPosition::default(),
            orientation: Orientation::default(),
            variant: V::default(),
        }
    }
}

impl<V> StyleQuery<V> {
    /// This query with the state `state`.
    #[must_use]
    pub fn state(mut self, state: WidgetState) -> Self {
        self.state = state;
        self
    }

    /// This query with `flag` added to its state.
    #[must_use]
    pub fn with_state(mut self, flag: WidgetState) -> Self {
        self.state |= flag;
        self
    }

    /// This query with `size`.
    #[must_use]
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// This query with `surface`.
    #[must_use]
    pub fn surface(mut self, surface: Surface) -> Self {
        self.surface = surface;
        self
    }

    /// This query with `position`.
    #[must_use]
    pub fn position(mut self, position: GroupPosition) -> Self {
        self.position = position;
        self
    }

    /// This query with `orientation`.
    #[must_use]
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// This query with `variant`.
    #[must_use]
    pub fn variant(mut self, variant: V) -> Self {
        self.variant = variant;
        self
    }
}

impl<V: Default> From<WidgetState> for StyleQuery<V> {
    fn from(state: WidgetState) -> Self {
        Self::new(state)
    }
}

/// A button's variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonVariant {
    /// The standard button.
    #[default]
    Standard,
    /// The primary (accent) button.
    Primary,
    /// A secondary button.
    Secondary,
    /// A destructive action's button.
    Destructive,
    /// An outlined button.
    Outlined,
    /// A flat (borderless) button.
    Flat,
    /// A glass button.
    Glass,
    /// A link-styled button.
    Link,
}

/// A button's shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ButtonShape {
    /// The platform's default shape.
    #[default]
    Default,
    /// Pill-shaped.
    Pill,
    /// A circle.
    Circle,
    /// A square.
    Square,
}

/// A button's variant axis: its variant and shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ButtonKind {
    /// The variant.
    pub variant: ButtonVariant,
    /// The shape.
    pub shape: ButtonShape,
}

/// The query of a button.
pub type ButtonQuery = StyleQuery<ButtonKind>;

impl ButtonQuery {
    /// A query of a primary button.
    #[must_use]
    pub fn primary() -> Self {
        Self::from(ButtonVariant::Primary)
    }

    /// A query of a secondary button.
    #[must_use]
    pub fn secondary() -> Self {
        Self::from(ButtonVariant::Secondary)
    }

    /// A query of a destructive button.
    #[must_use]
    pub fn destructive() -> Self {
        Self::from(ButtonVariant::Destructive)
    }

    /// A query of an outlined button.
    #[must_use]
    pub fn outlined() -> Self {
        Self::from(ButtonVariant::Outlined)
    }

    /// A query of a flat button.
    #[must_use]
    pub fn flat() -> Self {
        Self::from(ButtonVariant::Flat)
    }

    /// A query of a glass button.
    #[must_use]
    pub fn glass() -> Self {
        Self::from(ButtonVariant::Glass)
    }

    /// A query of a link-styled button.
    #[must_use]
    pub fn link() -> Self {
        Self::from(ButtonVariant::Link)
    }

    /// This query with the pill shape.
    #[must_use]
    pub fn pill(mut self) -> Self {
        self.variant.shape = ButtonShape::Pill;
        self
    }

    /// This query with the circle shape.
    #[must_use]
    pub fn circle(mut self) -> Self {
        self.variant.shape = ButtonShape::Circle;
        self
    }

    /// This query with the square shape.
    #[must_use]
    pub fn square(mut self) -> Self {
        self.variant.shape = ButtonShape::Square;
        self
    }
}

impl From<ButtonVariant> for ButtonQuery {
    fn from(variant: ButtonVariant) -> Self {
        Self::new(WidgetState::NONE).variant(ButtonKind {
            variant,
            shape: ButtonShape::default(),
        })
    }
}

/// A text input's variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InputVariant {
    /// The standard field.
    #[default]
    Standard,
    /// An outlined field.
    Outlined,
    /// A filled field.
    Filled,
    /// A search field.
    Search,
    /// A borderless field.
    Borderless,
}

/// The query of a text input.
pub type InputQuery = StyleQuery<InputVariant>;

impl From<InputVariant> for InputQuery {
    fn from(variant: InputVariant) -> Self {
        Self::new(WidgetState::NONE).variant(variant)
    }
}

/// A tab's variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TabVariant {
    /// The standard tab.
    #[default]
    Standard,
    /// A document tab.
    Document,
    /// A pill tab.
    Pill,
    /// A segment of a segmented control.
    Segmented,
}

/// The query of a tab.
pub type TabQuery = StyleQuery<TabVariant>;

impl From<TabVariant> for TabQuery {
    fn from(variant: TabVariant) -> Self {
        Self::new(WidgetState::NONE).variant(variant)
    }
}

/// A list's variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ListVariant {
    /// A plain list.
    #[default]
    Plain,
    /// An inset grouped list.
    InsetGrouped,
    /// A sidebar list.
    Sidebar,
    /// A table.
    Table,
}

/// A list row's variant axis: the list's variant and whether the row is an
/// alternate one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ListKind {
    /// The list's variant.
    pub variant: ListVariant,
    /// Whether the row is an alternate (striped) one.
    pub alternate: bool,
}

/// The query of a list row.
pub type ListQuery = StyleQuery<ListKind>;

impl ListQuery {
    /// This query with the row marked alternate or not.
    #[must_use]
    pub fn alternate(mut self, alternate: bool) -> Self {
        self.variant.alternate = alternate;
        self
    }
}

impl From<ListVariant> for ListQuery {
    fn from(variant: ListVariant) -> Self {
        Self::new(WidgetState::NONE).variant(ListKind {
            variant,
            alternate: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_state_bit_operators_and_not_masking() {
        let s = WidgetState::HOVER | WidgetState::CHECKED;
        assert!(s.contains(WidgetState::HOVER));
        assert!(!s.contains(WidgetState::HOVER | WidgetState::PRESSED));
        assert!(s.intersects(WidgetState::HOVER | WidgetState::PRESSED));
        assert!(!s.intersects(WidgetState::PRESSED));
        assert_eq!(s & WidgetState::CHECKED, WidgetState::CHECKED);
        assert_eq!(
            s.union(WidgetState::OPEN).bits(),
            (1 << 0) | (1 << 7) | (1 << 10)
        );
        assert_eq!(s.difference(WidgetState::HOVER), WidgetState::CHECKED);
        assert_eq!(!WidgetState::NONE, WidgetState::ALL);
        assert_eq!((!s).bits() >> 19, 0);
        assert_eq!(!s & s, WidgetState::NONE);
        assert_eq!((!s) | s, WidgetState::ALL);
        assert_eq!(WidgetState::from_bits_truncate(u32::MAX), WidgetState::ALL);
        assert_eq!(
            WidgetState::ALL.bits(),
            WidgetState::HOVER_WITHIN.bits() * 2 - 1
        );
        let mut m = WidgetState::NONE;
        m |= WidgetState::DISABLED;
        m |= WidgetState::FOCUSED;
        m &= !WidgetState::FOCUSED;
        assert_eq!(m, WidgetState::DISABLED);
        assert!(WidgetState::default().is_empty());
    }

    #[test]
    fn theme_context_defaults_and_builders() {
        let light = ThemeContext::light();
        assert_eq!(light, ThemeContext::default());
        assert!(light.window_active);
        assert_eq!(light.text_scale, 1.0);
        assert_eq!(ThemeContext::dark().scheme, Scheme::Dark);
        let c = ThemeContext::light()
            .with_contrast(Contrast::High)
            .with_motion(MotionPreference::Reduced)
            .with_reduced_transparency(true)
            .with_bold_text(true)
            .with_button_shapes(true)
            .with_differentiate_without_color(true)
            .with_window_active(false)
            .with_direction(Direction::Rtl);
        assert_eq!(c.contrast, Contrast::High);
        assert_eq!(c.motion, MotionPreference::Reduced);
        assert!(c.reduced_transparency && c.bold_text && c.button_shapes);
        assert!(c.differentiate_without_color && !c.window_active);
        assert_eq!(c.direction, Direction::Rtl);
        assert_eq!(light.clone().with_text_scale(9.0).text_scale, 4.0);
        assert_eq!(light.clone().with_text_scale(0.1).text_scale, 0.5);
        assert_eq!(light.clone().with_text_scale(2.0).text_scale, 2.0);
        assert_eq!(light.clone().with_text_scale(f32::NAN).text_scale, 1.0);
        assert_eq!(light.clone().with_text_scale(f32::INFINITY).text_scale, 1.0);
        assert_eq!(light.with_text_scale(f32::NEG_INFINITY).text_scale, 1.0);
    }

    #[test]
    fn style_query_builders_and_helpers() {
        let q = ButtonQuery::primary()
            .pill()
            .size(ControlSize::Large)
            .state(WidgetState::HOVER)
            .with_state(WidgetState::FOCUSED)
            .surface(Surface::Card)
            .position(GroupPosition::Last)
            .orientation(Orientation::Vertical);
        assert_eq!(q.variant.variant, ButtonVariant::Primary);
        assert_eq!(q.variant.shape, ButtonShape::Pill);
        assert_eq!(q.size, ControlSize::Large);
        assert_eq!(q.state, WidgetState::HOVER | WidgetState::FOCUSED);
        assert_eq!(q.surface, Surface::Card);
        assert_eq!(q.position, GroupPosition::Last);
        assert_eq!(q.orientation, Orientation::Vertical);
        assert_eq!(
            ButtonQuery::link().circle().variant.shape,
            ButtonShape::Circle
        );
        assert_eq!(
            ButtonQuery::flat().square().variant.shape,
            ButtonShape::Square
        );
        for (q, v) in [
            (ButtonQuery::secondary(), ButtonVariant::Secondary),
            (ButtonQuery::destructive(), ButtonVariant::Destructive),
            (ButtonQuery::outlined(), ButtonVariant::Outlined),
            (ButtonQuery::glass(), ButtonVariant::Glass),
        ] {
            assert_eq!(q.variant.variant, v);
        }
        let plain: StyleQuery = WidgetState::PRESSED.into();
        assert_eq!(plain, StyleQuery::new(WidgetState::PRESSED));
        assert_eq!(
            InputQuery::from(InputVariant::Search).variant,
            InputVariant::Search
        );
        assert_eq!(TabQuery::from(TabVariant::Pill).variant, TabVariant::Pill);
        let row = ListQuery::from(ListVariant::Sidebar).alternate(true);
        assert_eq!(row.variant.variant, ListVariant::Sidebar);
        assert!(row.variant.alternate);
    }
}
