//! What S8's query evaluates with: the element a part is queried for, the
//! scope it is queried in, and the toolkit's font measurement.

use super::blocks::part_style::RoleRef;
use super::blocks::text::{LanguageTag, ResolvedFont};
use super::color::Color;
use super::roles::SurfaceSet;
use super::selector::{
    ControlSize, Density, Direction, PartRef, SizeClasses, StateFlag, StateFlags, VariantRef,
};
use super::units::{Fraction, Px, Size};

/// The toolkit's font measurement (S8), all infallible.
pub trait FontMetrics {
    /// The average character width.
    fn char_width(&self, font: &ResolvedFont) -> Px;
    /// The widest glyph's width (KCharSelect's).
    fn max_char_width(&self, font: &ResolvedFont) -> Px;
    /// The x-height.
    fn x_height(&self, font: &ResolvedFont) -> Px;
    /// Ascent plus descent.
    fn height(&self, font: &ResolvedFont) -> Px;
    /// The line spacing: ascent plus descent plus leading.
    fn line_spacing(&self, font: &ResolvedFont) -> Px;
    /// The font's own underline offset, in `Decoration`'s convention (S2.8).
    fn underline_offset(&self, font: &ResolvedFont) -> Px;
    /// The font's own underline thickness.
    fn underline_thickness(&self, font: &ResolvedFont) -> Px;
    /// The font's own strikeout offset, in `Decoration`'s convention.
    fn strikeout_offset(&self, font: &ResolvedFont) -> Px;
    /// The font's own strikeout thickness.
    fn strikeout_thickness(&self, font: &ResolvedFont) -> Px;
}

/// One part a scope passes through: the element's part and what the
/// context selectors match on, and the bindings it makes (S3.3, S8).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ScopeEntry {
    /// The part.
    pub(crate) part: PartRef,
    /// Its element's variants.
    pub(crate) variants: Vec<VariantRef>,
    /// Its element's states, reduced by its widget's exclusive groups.
    pub(crate) states: StateFlags,
    /// Its element's control size.
    pub(crate) size: Option<ControlSize>,
    /// The surface set it binds.
    pub(crate) surface: Option<SurfaceSet>,
    /// The roles it binds, each evaluated in the scope outside it; `None`:
    /// bound to a role not drawn.
    pub(crate) roles: Vec<(RoleRef, Option<Color>)>,
}

/// The scope a query is evaluated in (S8): owned, evaluated values only,
/// so one kept across a re-resolution is safe to use but stale.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Scope {
    /// The parts the scope passes through, outermost first.
    pub(crate) entries: Vec<ScopeEntry>,
    /// The density.
    pub(crate) density: Option<Density>,
    /// The size classes.
    pub(crate) size_classes: SizeClasses,
    /// The direction.
    pub(crate) direction: Direction,
    /// The language, for icon lookup.
    pub(crate) language: Option<LanguageTag>,
    /// The window's size.
    pub(crate) window: Size,
    /// The screen's work area.
    pub(crate) screen: Size,
}

impl Scope {
    /// A root scope: no part entered, the window's and screen's sizes and
    /// the direction given.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "S8's query builds the scopes (M1a task 11b)")
    )]
    pub(crate) fn root(window: Size, screen: Size, direction: Direction) -> Scope {
        Scope {
            entries: Vec::new(),
            density: None,
            size_classes: SizeClasses::default(),
            direction,
            language: None,
            window,
            screen,
        }
    }

    /// The scope with `entry` entered, nearest last.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "S8's query builds the scopes (M1a task 11b)")
    )]
    pub(crate) fn entered(&self, entry: ScopeEntry) -> Scope {
        let mut out = self.clone();
        out.entries.push(entry);
        out
    }
}

/// A sibling of the queried element (S8).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Sibling<'a> {
    /// Its part.
    pub part: PartRef,
    /// Its variants.
    pub variants: &'a [VariantRef],
    /// Its states.
    pub states: StateFlags,
}

impl<'a> Sibling<'a> {
    /// The sibling of `part`, holding `variants` and `states`.
    #[must_use]
    pub fn new(part: PartRef, variants: &'a [VariantRef], states: StateFlags) -> Sibling<'a> {
        Sibling {
            part,
            variants,
            states,
        }
    }
}

/// The data an element's values may read (S8, R25.3).
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct ElementData<'a> {
    /// The laid-out box's width.
    pub width: Px,
    /// The laid-out box's height.
    pub height: Px,
    /// The progress.
    pub progress: Fraction,
    /// The scroll offset.
    pub scroll_offset: Px,
    /// The scroll progress.
    pub scroll_progress: Fraction,
    /// The item's index.
    pub item_index: u32,
    /// The number of items.
    pub item_count: u32,
    /// The text.
    pub text: &'a str,
    /// A colour the connector supplies that derived values follow.
    pub color: Color,
    /// How far a `Move` presentation moves the element.
    pub move_distance: Px,
}

/// Zero size, progress 0, scroll 0, item 0 of 1, empty text, transparent
/// (S8).
impl Default for ElementData<'_> {
    fn default() -> Self {
        ElementData {
            width: Px(0.0),
            height: Px(0.0),
            progress: Fraction(0.0),
            scroll_offset: Px(0.0),
            scroll_progress: Fraction(0.0),
            item_index: 0,
            item_count: 1,
            text: "",
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            move_distance: Px(0.0),
        }
    }
}

/// The element a part is queried for (S8).
#[derive(Clone)]
#[non_exhaustive]
pub struct Element<'a> {
    /// The part; it names the widget too.
    pub part: PartRef,
    /// The scope returned by the query of the innermost part that encloses
    /// it in the catalogue's nesting.
    pub scope: &'a Scope,
    /// Its variants.
    pub variants: &'a [VariantRef],
    /// Its control size; `None`: the declared default size.
    pub size: Option<ControlSize>,
    /// Its own density, passed to its subtree.
    pub density: Option<Density>,
    /// Its own direction, passed to its subtree.
    pub direction: Option<Direction>,
    /// Its own size classes, passed to its subtree.
    pub size_classes: Option<SizeClasses>,
    /// Its language, for icon lookup.
    pub language: Option<LanguageTag>,
    /// Its states.
    pub states: StateFlags,
    /// The order the connector last set its flags in, the latest last.
    pub state_order: &'a [StateFlag],
    /// All its siblings, itself at `index`.
    pub siblings: &'a [Sibling<'a>],
    /// Its place among `siblings`.
    pub index: usize,
    /// Its data.
    pub data: ElementData<'a>,
    /// The toolkit's measurement.
    pub metrics: &'a dyn FontMetrics,
}

impl<'a> Element<'a> {
    /// The element of `part` in `scope`, measured by `metrics`: no variants,
    /// the default size, density and direction, no states, no siblings, the
    /// default data.
    #[must_use]
    pub fn new(part: PartRef, scope: &'a Scope, metrics: &'a dyn FontMetrics) -> Element<'a> {
        Element {
            part,
            scope,
            variants: &[],
            size: None,
            density: None,
            direction: None,
            size_classes: None,
            language: None,
            states: StateFlags::new(),
            state_order: &[],
            siblings: &[],
            index: 0,
            data: ElementData::default(),
            metrics,
        }
    }

    /// The element with its variants.
    #[must_use]
    pub fn with_variants(mut self, variants: &'a [VariantRef]) -> Element<'a> {
        self.variants = variants;
        self
    }

    /// The element at a control size.
    #[must_use]
    pub fn with_size(mut self, size: ControlSize) -> Element<'a> {
        self.size = Some(size);
        self
    }

    /// The element with its own density, passed to its subtree.
    #[must_use]
    pub fn with_density(mut self, density: Density) -> Element<'a> {
        self.density = Some(density);
        self
    }

    /// The element with its own direction, passed to its subtree.
    #[must_use]
    pub fn with_direction(mut self, direction: Direction) -> Element<'a> {
        self.direction = Some(direction);
        self
    }

    /// The element with its own size classes, passed to its subtree.
    #[must_use]
    pub fn with_size_classes(mut self, size_classes: SizeClasses) -> Element<'a> {
        self.size_classes = Some(size_classes);
        self
    }

    /// The element with its language, for icon lookup.
    #[must_use]
    pub fn with_language(mut self, language: LanguageTag) -> Element<'a> {
        self.language = Some(language);
        self
    }

    /// The element holding `states`, set in `state_order` (the latest last).
    #[must_use]
    pub fn with_states(mut self, states: StateFlags, state_order: &'a [StateFlag]) -> Element<'a> {
        self.states = states;
        self.state_order = state_order;
        self
    }

    /// The element at `index` among all its `siblings`.
    #[must_use]
    pub fn with_siblings(mut self, siblings: &'a [Sibling<'a>], index: usize) -> Element<'a> {
        self.siblings = siblings;
        self.index = index;
        self
    }

    /// The element with its data.
    #[must_use]
    pub fn with_data(mut self, data: ElementData<'a>) -> Element<'a> {
        self.data = data;
        self
    }
}
