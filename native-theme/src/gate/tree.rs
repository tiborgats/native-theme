//! A resolved value as the gate compares it (S17): read from the value's
//! `Debug` form, which every resolved type derives, so one comparison
//! serves every type the model holds, its numeric and colour leaves within
//! a tolerance and every other leaf exactly.

use std::borrow::Cow;
use std::fmt;

/// A resolved value's tree: a struct's fields by name, a tuple's or a
/// list's items in order, and its leaves. `Some(x)` reads as `x`, so a
/// value and its absence differ by a leaf `None`; but a `Some` holding a
/// leaf `None` stays itself, so it differs from the absence `None`: a
/// `Value<Option<T>>`'s stated `None` (S6.1), and as the `Debug` form
/// writes it alike, a unit variant named `None` (`Some(Wrap::None)`), each
/// `"none"` apart from `"absent"` (S11).
#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    /// A number.
    Number(f64),
    /// A colour: red, green, blue and alpha, each a fraction.
    Color([f64; 4]),
    /// A text, as written.
    Text(String),
    /// A name: a unit variant, `true`, `false`, `None`.
    Word(String),
    /// A struct or a variant with named fields.
    Fields {
        /// The type's or variant's name.
        name: String,
        /// The fields, in order.
        fields: Vec<(String, Tree)>,
    },
    /// A tuple, a tuple struct or a tuple variant; `name` empty for a
    /// tuple.
    Tuple {
        /// The type's or variant's name.
        name: String,
        /// The items, in order.
        items: Vec<Tree>,
    },
    /// A list.
    List(Vec<Tree>),
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Number(x) => write!(f, "{x}"),
            Tree::Color([r, g, b, a]) => write!(f, "rgba({r}, {g}, {b}, {a})"),
            Tree::Text(text) => write!(f, "{text:?}"),
            Tree::Word(word) => f.write_str(word),
            Tree::Fields { name, fields } => {
                write!(f, "{name} {{ ")?;
                for (index, (field, value)) in fields.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{field}: {value}")?;
                }
                f.write_str(" }")
            }
            Tree::Tuple { name, items } => {
                write!(f, "{name}(")?;
                list(f, items)?;
                f.write_str(")")
            }
            Tree::List(items) => {
                f.write_str("[")?;
                list(f, items)?;
                f.write_str("]")
            }
        }
    }
}

fn list(f: &mut fmt::Formatter<'_>, items: &[Tree]) -> fmt::Result {
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            f.write_str(", ")?;
        }
        write!(f, "{item}")?;
    }
    Ok(())
}

/// One token of a `Debug` form, its words and texts borrowed from the form
/// where no escape changes them.
#[derive(Clone, Debug, PartialEq)]
enum Token<'t> {
    Ident(&'t str),
    Number(f64),
    Text(Cow<'t, str>),
    Punct(char),
}

/// The tokens of `text`, or `None` where it holds what no derived `Debug`
/// writes.
fn tokens(text: &str) -> Option<Vec<Token<'_>>> {
    let mut out = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '{' | '}' | '(' | ')' | '[' | ']' | ',' | ':' => out.push(Token::Punct(c)),
            '"' => {
                let from = start.checked_add(1)?;
                let mut value: Option<String> = None;
                loop {
                    let (at, c) = chars.next()?;
                    match c {
                        '"' => {
                            out.push(Token::Text(match value {
                                Some(value) => Cow::Owned(value),
                                None => Cow::Borrowed(text.get(from..at)?),
                            }));
                            break;
                        }
                        '\\' => {
                            let held = value.get_or_insert_with(|| {
                                text.get(from..at).unwrap_or_default().to_owned()
                            });
                            let (_, escaped) = chars.next()?;
                            held.push(match escaped {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '0' => '\0',
                                other => other,
                            });
                        }
                        other => {
                            if let Some(held) = value.as_mut() {
                                held.push(other);
                            }
                        }
                    }
                }
            }
            c if c.is_ascii_digit() || c == '-' => {
                let mut end = start.checked_add(c.len_utf8())?;
                while let Some(&(at, next)) = chars.peek() {
                    let exponent_sign = (next == '-' || next == '+')
                        && text.get(..at).is_some_and(|before| before.ends_with('e'));
                    if next.is_ascii_alphanumeric() || next == '.' || exponent_sign {
                        end = at.checked_add(next.len_utf8())?;
                        chars.next();
                    } else {
                        break;
                    }
                }
                let written = text.get(start..end)?;
                let number = match written {
                    "-inf" => f64::NEG_INFINITY,
                    _ => written.parse::<f64>().ok()?,
                };
                out.push(Token::Number(number));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut end = start.checked_add(c.len_utf8())?;
                while let Some(&(at, next)) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        end = at.checked_add(next.len_utf8())?;
                        chars.next();
                    } else {
                        break;
                    }
                }
                let word = text.get(start..end)?;
                out.push(match word {
                    "inf" => Token::Number(f64::INFINITY),
                    "NaN" => Token::Number(f64::NAN),
                    _ => Token::Ident(word),
                });
            }
            _ => return None,
        }
    }
    Some(out)
}

/// A reader over a `Debug` form's tokens.
struct Parser<'t> {
    tokens: Vec<Token<'t>>,
    at: usize,
    /// Where each value skipped ends, by where it starts (`usize::MAX`:
    /// not skipped yet), so it is read once.
    ends: Vec<usize>,
}

impl<'t> Parser<'t> {
    fn peek(&self) -> Option<&Token<'t>> {
        self.tokens.get(self.at)
    }

    fn next(&mut self) -> Option<&Token<'t>> {
        let token = self.tokens.get(self.at);
        self.at = self.at.saturating_add(1);
        token
    }

    fn punct(&mut self, c: char) -> Option<()> {
        (*self.next()? == Token::Punct(c)).then_some(())
    }

    fn at_punct(&self, c: char) -> bool {
        self.peek() == Some(&Token::Punct(c))
    }

    /// Items up to `close`, separated by commas, a trailing one allowed.
    fn items(&mut self, close: char) -> Option<Vec<Tree>> {
        let mut items = Vec::new();
        while !self.at_punct(close) {
            items.push(self.value()?);
            if !self.at_punct(close) {
                self.punct(',')?;
            }
        }
        self.punct(close)?;
        Some(items)
    }

    fn value(&mut self) -> Option<Tree> {
        match self.next()?.clone() {
            Token::Number(x) => Some(Tree::Number(x)),
            Token::Text(text) => Some(Tree::Text(text.into_owned())),
            Token::Punct('[') => self.items(']').map(Tree::List),
            Token::Punct('(') => self.items(')').map(|items| Tree::Tuple {
                name: String::new(),
                items,
            }),
            Token::Punct(_) => None,
            Token::Ident(name) => {
                if self.at_punct('{') {
                    self.punct('{')?;
                    let mut fields = Vec::new();
                    while !self.at_punct('}') {
                        let Token::Ident(field) = self.next()?.clone() else {
                            return None;
                        };
                        self.punct(':')?;
                        fields.push((field.to_owned(), self.value()?));
                        if !self.at_punct('}') {
                            self.punct(',')?;
                        }
                    }
                    self.punct('}')?;
                    Some(fields_tree(name.to_owned(), fields))
                } else if self.at_punct('(') {
                    self.punct('(')?;
                    let mut items = self.items(')')?;
                    if name == "Some"
                        && let [item] = items.as_slice()
                        && *item != Tree::Word("None".to_owned())
                    {
                        return items.pop();
                    }
                    Some(Tree::Tuple {
                        name: name.to_owned(),
                        items,
                    })
                } else {
                    Some(Tree::Word(name.to_owned()))
                }
            }
        }
    }

    /// Skips the items up to `close`, as [`Parser::items`] reads them, each
    /// item's start kept.
    fn skip_items(&mut self, close: char) -> Option<Vec<usize>> {
        let mut starts = Vec::new();
        while !self.at_punct(close) {
            starts.push(self.at);
            self.skip()?;
            if !self.at_punct(close) {
                self.punct(',')?;
            }
        }
        self.punct(close)?;
        Some(starts)
    }

    /// Skips one value, as [`Parser::value`] reads it, each value read
    /// once.
    fn skip(&mut self) -> Option<()> {
        let start = self.at;
        if let Some(end) = self
            .ends
            .get(start)
            .copied()
            .filter(|end| *end != usize::MAX)
        {
            self.at = end;
            return Some(());
        }
        self.skip_once()?;
        if let Some(end) = self.ends.get_mut(start) {
            *end = self.at;
        }
        Some(())
    }

    fn skip_once(&mut self) -> Option<()> {
        match self.next()? {
            Token::Number(_) | Token::Text(_) => Some(()),
            Token::Punct('[') => self.skip_items(']').map(drop),
            Token::Punct('(') => self.skip_items(')').map(drop),
            Token::Punct(_) => None,
            Token::Ident(_) => {
                if self.at_punct('{') {
                    self.skip_fields().map(drop)
                } else if self.at_punct('(') {
                    self.punct('(')?;
                    self.skip_items(')').map(drop)
                } else {
                    Some(())
                }
            }
        }
    }

    /// Skips a struct's fields from its `{`, each field's name and its
    /// value's start kept.
    fn skip_fields(&mut self) -> Option<Vec<(&'t str, usize)>> {
        self.punct('{')?;
        let mut fields = Vec::new();
        while !self.at_punct('}') {
            let Token::Ident(field) = *self.next()? else {
                return None;
            };
            self.punct(':')?;
            fields.push((field, self.at));
            self.skip()?;
            if !self.at_punct('}') {
                self.punct(',')?;
            }
        }
        self.punct('}')?;
        Some(fields)
    }

    /// The value at `start` read whole, the reader left where it was.
    fn value_from(&mut self, start: usize) -> Option<Tree> {
        let held = std::mem::replace(&mut self.at, start);
        let value = self.value();
        self.at = held;
        value
    }

    /// The subtree `path` names in the value at `start`, as [`Tree::at`]
    /// finds it in the value's tree, the reader left where it was: `None`
    /// where it names nothing, or the value is malformed.
    fn at_path(&mut self, start: usize, path: &[String]) -> Option<Tree> {
        let Some((first, rest)) = path.split_first() else {
            return self.value_from(start);
        };
        let held = std::mem::replace(&mut self.at, start);
        let found = self.step(first, rest, path);
        self.at = held;
        found
    }

    fn step(&mut self, first: &str, rest: &[String], path: &[String]) -> Option<Tree> {
        let start = self.at;
        match *self.next()? {
            Token::Number(_) | Token::Text(_) => None,
            Token::Punct('[') => {
                for item in self.skip_items(']')? {
                    self.at = item;
                    if *self.next()? != Token::Punct('(') {
                        continue;
                    }
                    let [key, value] = self.skip_items(')')?[..] else {
                        continue;
                    };
                    let key = self.value_from(key)?;
                    if key.text() == Some(first)
                        && let Some(found) = self.at_path(value, rest)
                    {
                        return Some(found);
                    }
                }
                None
            }
            Token::Punct('(') => match self.skip_items(')')?[..] {
                [only] => self.at_path(only, path),
                _ => None,
            },
            Token::Punct(_) => None,
            Token::Ident(name) => {
                if self.at_punct('{') {
                    let fields = self.skip_fields()?;
                    // A colour reads as its four channels, a leaf.
                    if name == "Color" && matches!(self.value_from(start)?, Tree::Color(_)) {
                        return None;
                    }
                    match fields.iter().find(|(field, _)| *field == first) {
                        Some((_, at)) => self.at_path(*at, rest),
                        None if snake(name) == first => self.at_path(start, rest),
                        None => None,
                    }
                } else if self.at_punct('(') {
                    self.punct('(')?;
                    let [only] = self.skip_items(')')?[..] else {
                        return None;
                    };
                    let transparent =
                        name == "Some" && self.value_from(only)? != Tree::Word("None".to_owned());
                    if transparent {
                        self.at_path(only, path)
                    } else if snake(name) == first {
                        self.at_path(only, rest)
                    } else {
                        self.at_path(only, path)
                    }
                } else {
                    None
                }
            }
        }
    }
}

/// A struct's tree, a colour's its four channels.
fn fields_tree(name: String, fields: Vec<(String, Tree)>) -> Tree {
    if name == "Color"
        && let [
            (r, Tree::Number(red)),
            (g, Tree::Number(green)),
            (b, Tree::Number(blue)),
            (a, Tree::Number(alpha)),
        ] = fields.as_slice()
        && [r, g, b, a].map(String::as_str) == ["r", "g", "b", "a"]
    {
        return Tree::Color([*red, *green, *blue, *alpha]);
    }
    Tree::Fields { name, fields }
}

impl Tree {
    /// The tree of a value's `Debug` form; `None` where the form is not one
    /// a derived `Debug` writes.
    pub fn of(value: &dyn fmt::Debug) -> Option<Tree> {
        let text = format!("{value:?}");
        let mut parser = Parser {
            tokens: tokens(&text)?,
            at: 0,
            ends: Vec::new(),
        };
        let tree = parser.value()?;
        parser.peek().is_none().then_some(tree)
    }

    /// [`Tree::of`]'s subtree at `path` ([`Tree::at`]), built alone: the
    /// rest of the form is read, never built.
    pub fn of_at(value: &dyn fmt::Debug, path: &[String]) -> Option<Tree> {
        let text = format!("{value:?}");
        let tokens = tokens(&text)?;
        let mut parser = Parser {
            ends: vec![usize::MAX; tokens.len()],
            tokens,
            at: 0,
        };
        parser.skip()?;
        if parser.peek().is_some() {
            return None;
        }
        parser.at_path(0, path)
    }

    /// The subtree `path` names, in the model's Rust names (S1): a field by
    /// its name, a variant by its name in snake_case, a map's entry by its
    /// key; `None` where it names nothing.
    pub(crate) fn at(&self, path: &[String]) -> Option<&Tree> {
        let Some((first, rest)) = path.split_first() else {
            return Some(self);
        };
        match self {
            Tree::Fields { name, fields } => match fields.iter().find(|(held, _)| held == first) {
                Some((_, value)) => value.at(rest),
                None if snake(name) == *first => self.at(rest),
                None => None,
            },
            Tree::Tuple { name, items } if snake(name) == *first => match items.as_slice() {
                [only] => only.at(rest),
                _ => None,
            },
            Tree::Tuple { items, .. } => match items.as_slice() {
                [only] => only.at(path),
                _ => None,
            },
            Tree::List(items) => items.iter().find_map(|item| match item {
                Tree::Tuple { name, items } if name.is_empty() => match items.as_slice() {
                    [key, value] if key.text() == Some(first.as_str()) => value.at(rest),
                    _ => None,
                },
                _ => None,
            }),
            Tree::Number(_) | Tree::Color(_) | Tree::Text(_) | Tree::Word(_) => None,
        }
    }

    /// [`Tree::at`], mutably.
    fn at_mut(&mut self, path: &[String]) -> Option<&mut Tree> {
        /// The step [`Tree::at`] takes at the first segment.
        enum Step {
            Field(usize),
            Named,
            Only { consumed: bool },
            Entry(usize),
        }
        let Some((first, rest)) = path.split_first() else {
            return Some(self);
        };
        let step = match &*self {
            Tree::Fields { name, fields } => {
                match fields.iter().position(|(held, _)| held == first) {
                    Some(index) => Step::Field(index),
                    None if snake(name) == *first => Step::Named,
                    None => return None,
                }
            }
            Tree::Tuple { name, items } if items.len() == 1 => Step::Only {
                consumed: snake(name) == *first,
            },
            Tree::List(items) => Step::Entry(items.iter().position(|item| match item {
                Tree::Tuple { name, items } if name.is_empty() => match items.as_slice() {
                    [key, value] => key.text() == Some(first.as_str()) && value.at(rest).is_some(),
                    _ => false,
                },
                _ => false,
            })?),
            _ => return None,
        };
        match (step, self) {
            (Step::Field(index), Tree::Fields { fields, .. }) => fields
                .get_mut(index)
                .and_then(|(_, value)| value.at_mut(rest)),
            (Step::Named, tree) => tree.at_mut(rest),
            (Step::Only { consumed }, Tree::Tuple { items, .. }) => {
                let only = items.first_mut()?;
                if consumed {
                    only.at_mut(rest)
                } else {
                    only.at_mut(path)
                }
            }
            (Step::Entry(index), Tree::List(items)) => match items.get_mut(index)? {
                Tree::Tuple { items, .. } => items.get_mut(1)?.at_mut(rest),
                _ => None,
            },
            _ => None,
        }
    }

    /// `self` with the subtree at `path` replaced by `from`'s there, where
    /// both hold one: a place compared by another key's fact (S17).
    pub(crate) fn graft(&mut self, path: &[String], from: &Tree) {
        if let (Some(place), Some(value)) = (self.at_mut(path), from.at(path)) {
            *place = value.clone();
        }
    }

    /// The text a map's key holds: a text, or a newtype of one (`Key`).
    fn text(&self) -> Option<&str> {
        match self {
            Tree::Text(text) | Tree::Word(text) => Some(text),
            Tree::Tuple { items, .. } => match items.as_slice() {
                [only] => only.text(),
                _ => None,
            },
            _ => None,
        }
    }
}

/// A type's or variant's name in snake_case, as a path writes it.
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (index, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// How closely a fact's value must hold (S17 *The gate's tolerance*): a
/// measured tolerance in the value's canonical unit, one for every numeric
/// leaf or one per leaf, or, without one, the quote check's precision.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tolerance {
    /// The measured tolerance for every numeric leaf, where one is stated
    /// so.
    every: Option<f64>,
    /// The measured tolerance per leaf, by its path in the value as S11
    /// spells it, each unit suffix left out.
    leaves: Vec<(Vec<String>, f64)>,
    /// The leaves whose value is in points (S9.4), by their paths as
    /// `leaves` holds them: their tolerance is in points, the resolved
    /// value in px.
    points: Vec<Vec<String>>,
    /// The px a point resolves to where the value is compared; `None`
    /// before the gate knows the place.
    px_per_point: Option<f64>,
    /// A measured citation read from drawn pixels serves it (S17: an
    /// image's analysis, or a render-node file): its colours compare as the
    /// renderer draws the resolved one. A measured value an API read (an
    /// `NSColor`, a kdeglobals key, a probe's own value) compares as read.
    from_pixels: bool,
    /// How the measured platform's renderer draws a colour, where its
    /// colours were measured from drawn pixels and the gate knows the
    /// platform: the resolved colour compared as drawn (S17, ADD-A131).
    drawn: Option<Drawing>,
}

/// How the measured platform's renderer draws a resolved colour into the
/// capture's frame, which the gate applies before comparing a colour
/// measured from drawn pixels (S17, S2.1's hand-over; ADD-A131). An in-gamut
/// colour draws the same in every case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drawing {
    /// Premultiplied, each premultiplied channel clamped to `0..=1`: a
    /// channel `c` at alpha `a > 0` draws as `clamp(c·a, 0, 1) / a`, the
    /// alpha unchanged, and at alpha 0 the alpha alone, as transparent
    /// black. GTK's (`adwaita`) and Qt's raster engine, QPainter's
    /// (`kde-breeze`).
    Premultiplied,
    /// Each straight channel and the alpha clamped to `0..=1` before
    /// anything draws: Compose's sRGB `Color` (`material-3`).
    Clamped,
    /// No source states how the renderer draws a channel outside `0..=1`
    /// (`macos-*`, `ios-*`, `windows-11`, Qt Quick's scene graph): a colour
    /// out of the gamut fails, an in-gamut one compares as it is.
    Unread,
}

impl Drawing {
    /// `color`, red, green, blue and alpha, as the renderer draws it;
    /// `None` where it leaves the gamut and no source states the drawing.
    fn draw(self, [r, g, b, a]: [f64; 4]) -> Option<[f64; 4]> {
        match self {
            Drawing::Premultiplied => {
                let a = a.clamp(0.0, 1.0);
                if a <= 0.0 {
                    return Some([0.0; 4]);
                }
                let channel = |c: f64| (c * a).clamp(0.0, 1.0) / a;
                Some([channel(r), channel(g), channel(b), a])
            }
            Drawing::Clamped => Some([r, g, b, a].map(|c| c.clamp(0.0, 1.0))),
            Drawing::Unread => [r, g, b, a]
                .iter()
                .all(|c| (0.0..=1.0).contains(c))
                .then_some([r, g, b, a]),
        }
    }
}

/// The quote check's relative precision for numbers (S17).
const NUMBER_PRECISION: f64 = 1e-4;

/// The quote check's precision for a colour's channels, in 8-bit units
/// (S17).
const COLOR_PRECISION: f64 = 1e-3;

/// The units a tolerance table's leaf may be suffixed by (S11).
const SUFFIXES: [&str; 4] = ["_px", "_ms", "_deg", "_pt"];

impl Tolerance {
    /// The quote check's precision: numbers to a relative 1e-4, colours as
    /// 8-bit channels, alpha included, to 1e-3.
    #[must_use]
    pub fn quote() -> Tolerance {
        Tolerance::default()
    }

    /// One measured tolerance for every numeric leaf.
    #[must_use]
    pub fn every(tolerance: f64) -> Tolerance {
        Tolerance {
            every: Some(tolerance),
            ..Tolerance::default()
        }
    }

    /// A measured tolerance per leaf, by the leaf's path in the value as
    /// S11 spells it (`layers.0.y_px`); a leaf suffixed `_pt` is in points.
    #[must_use]
    pub fn per_leaf(leaves: Vec<(String, f64)>) -> Tolerance {
        let mut points = Vec::new();
        let leaves = leaves
            .into_iter()
            .map(|(path, tolerance)| {
                let segments = unsuffixed(&path);
                if path.ends_with("_pt") && !points.contains(&segments) {
                    points.push(segments.clone());
                }
                (segments, tolerance)
            })
            .collect();
        Tolerance {
            leaves,
            points,
            ..Tolerance::default()
        }
    }

    /// The tolerance of a measured citation read from drawn pixels (S17):
    /// its colours compare as the measured platform's renderer draws the
    /// resolved one, once [`Tolerance::drawn_by`] names it.
    #[must_use]
    pub fn from_drawn_pixels(self) -> Tolerance {
        Tolerance {
            from_pixels: true,
            ..self
        }
    }

    /// The tolerance with the leaves at `paths` (as S11 spells them in the
    /// value, `""` the value itself) in points (S9.4): a measured tolerance
    /// there is in points, and is compared in px as the value is.
    #[must_use]
    pub fn in_points(mut self, paths: &[String]) -> Tolerance {
        for path in paths {
            let segments = unsuffixed(path);
            if !self.points.contains(&segments) {
                self.points.push(segments);
            }
        }
        self
    }

    /// The tolerance whose colours, where a measured citation read from
    /// drawn pixels serves it, are compared as `drawing` draws them (S17,
    /// ADD-A131): the measured platform's renderer; a value read otherwise,
    /// and the quote check's precision, unchanged.
    #[must_use]
    pub fn drawn_by(&self, drawing: Drawing) -> Tolerance {
        Tolerance {
            drawn: self.from_pixels.then_some(drawing),
            ..self.clone()
        }
    }

    /// The tolerance compared where a point resolves to `px_per_point` px.
    #[must_use]
    pub(crate) fn at_points_of(&self, px_per_point: f64) -> Tolerance {
        Tolerance {
            px_per_point: Some(px_per_point),
            ..self.clone()
        }
    }

    /// The smaller of two measured tolerances per leaf, where several
    /// citations serve one generation (S17).
    #[must_use]
    pub fn smallest(self, other: Tolerance) -> Tolerance {
        let every = match (self.every, other.every) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        let mut leaves = self.leaves;
        for (path, tolerance) in other.leaves {
            match leaves.iter_mut().find(|(held, _)| *held == path) {
                Some((_, held)) => *held = held.min(tolerance),
                None => leaves.push((path, tolerance)),
            }
        }
        let mut points = self.points;
        for path in other.points {
            if !points.contains(&path) {
                points.push(path);
            }
        }
        Tolerance {
            every,
            leaves,
            points,
            px_per_point: self.px_per_point,
            from_pixels: self.from_pixels || other.from_pixels,
            drawn: self.drawn.or(other.drawn),
        }
    }

    /// The measured tolerance at the leaf `path`, where one is stated, in
    /// the resolved value's unit.
    fn at(&self, path: &[Segment]) -> Option<f64> {
        let leaf = self
            .leaves
            .iter()
            .filter(|(held, _)| spelled(held, path))
            .map(|(_, tolerance)| *tolerance)
            .reduce(f64::min);
        let tolerance = match (self.every, leaf) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }?;
        match self.px_per_point {
            Some(px) if self.points.iter().any(|held| spelled(held, path)) => Some(tolerance * px),
            _ => Some(tolerance),
        }
    }
}

/// A leaf's path, `.`-separated, with each segment's unit suffix left out.
fn unsuffixed(path: &str) -> Vec<String> {
    if path.is_empty() {
        return Vec::new();
    }
    path.split('.')
        .map(|segment| {
            SUFFIXES
                .iter()
                .find_map(|suffix| segment.strip_suffix(suffix))
                .unwrap_or(segment)
                .to_owned()
        })
        .collect()
}

/// One segment of a leaf's path in the compared tree: a field's name or an
/// item's index, or a name S11 may spell or leave out there (a variant's or
/// a type's name in snake_case, `cubic_bezier`; `layers` before a list's
/// index, as S11 spells a shadow's layers, S2.5, whose resolved form is the
/// list alone).
#[derive(Clone, Debug, PartialEq)]
struct Segment {
    name: String,
    optional: bool,
}

/// Whether `spelling`, a path as S11 spells it, names the leaf at `path`:
/// its segments in order, each optional one of `path` written or left out.
fn spelled(spelling: &[String], path: &[Segment]) -> bool {
    match (spelling.split_first(), path.split_first()) {
        (_, None) => spelling.is_empty(),
        (None, Some(_)) => path.iter().all(|segment| segment.optional),
        (Some((first, rest)), Some((segment, after))) => {
            (segment.name == *first && spelled(rest, after))
                || (segment.optional && spelled(spelling, after))
        }
    }
}

/// `path` with `name` appended.
fn below(path: &[Segment], name: &str, optional: bool) -> Vec<Segment> {
    let mut out = path.to_vec();
    out.push(Segment {
        name: name.to_owned(),
        optional,
    });
    out
}

/// `path` with a type's or variant's `name` appended as an optional
/// segment, where it has one.
fn named(path: &[Segment], name: &str) -> Vec<Segment> {
    if name.is_empty() {
        path.to_vec()
    } else {
        below(path, &snake(name), true)
    }
}

/// A path as a failure names it: its fields and indices, `.` for the value
/// itself.
fn written(path: &[Segment]) -> String {
    let names: Vec<&str> = path
        .iter()
        .filter(|segment| !segment.optional)
        .map(|segment| segment.name.as_str())
        .collect();
    if names.is_empty() {
        ".".to_owned()
    } else {
        names.join(".")
    }
}

/// Whether two numbers agree: within the measured tolerance, else the
/// quote check's relative precision.
fn numbers_agree(a: f64, b: f64, measured: Option<f64>) -> bool {
    if a == b {
        return true;
    }
    let difference = (a - b).abs();
    let magnitude = a.abs().max(b.abs());
    match measured {
        Some(tolerance) => difference <= tolerance + STORED * magnitude,
        None => difference <= NUMBER_PRECISION * magnitude,
    }
}

/// The model stores its numbers and channels as `f32`: a measured
/// tolerance allows their rounding, relative to the value, beside it.
const STORED: f64 = f32::EPSILON as f64;

impl Tree {
    /// Where `self`, the theme's, and `fact`, the fact's, differ beyond
    /// `tolerance`: the first differing leaf's path, `None` where they
    /// agree.
    #[must_use]
    pub fn differs(&self, fact: &Tree, tolerance: &Tolerance) -> Option<String> {
        self.differs_at(fact, tolerance, &[])
    }

    fn differs_at(&self, fact: &Tree, tolerance: &Tolerance, path: &[Segment]) -> Option<String> {
        let here = || Some(written(path));
        match (self, fact) {
            (Tree::Number(a), Tree::Number(b)) => {
                (!numbers_agree(*a, *b, tolerance.at(path))).then(|| written(path))
            }
            (Tree::Color(a), Tree::Color(b)) => {
                let (a, b) = match tolerance.drawn {
                    None => (*a, *b),
                    Some(drawing) => match (drawing.draw(*a), drawing.draw(*b)) {
                        (Some(a), Some(b)) => (a, b),
                        _ => {
                            return Some(format!(
                                "{} (out of the gamut, where no source states how the renderer \
                                 draws it)",
                                written(path)
                            ));
                        }
                    },
                };
                let limit = tolerance.at(path).unwrap_or(COLOR_PRECISION);
                let close = a
                    .into_iter()
                    .zip(b)
                    .all(|(a, b)| a == b || ((a - b) * 255.0).abs() <= limit + STORED * 255.0);
                (!close).then(|| written(path))
            }
            (Tree::Text(a), Tree::Text(b)) | (Tree::Word(a), Tree::Word(b)) => {
                (a != b).then(|| written(path))
            }
            (
                Tree::Fields { name, fields },
                Tree::Fields {
                    name: other,
                    fields: others,
                },
            ) => {
                if name != other || fields.len() != others.len() {
                    return here();
                }
                let path = named(path, name);
                fields
                    .iter()
                    .zip(others)
                    .find_map(|((field, a), (other, b))| {
                        if field != other {
                            return Some(written(&path));
                        }
                        a.differs_at(b, tolerance, &below(&path, field, false))
                    })
            }
            (
                Tree::Tuple { name, items },
                Tree::Tuple {
                    name: other,
                    items: others,
                },
            ) => {
                if name != other || items.len() != others.len() {
                    return here();
                }
                let path = named(path, name);
                if let ([a], [b]) = (items.as_slice(), others.as_slice()) {
                    return a.differs_at(b, tolerance, &path);
                }
                items
                    .iter()
                    .zip(others)
                    .enumerate()
                    .find_map(|(index, (a, b))| {
                        a.differs_at(b, tolerance, &below(&path, &index.to_string(), false))
                    })
            }
            (Tree::List(items), Tree::List(others)) => {
                if items.len() != others.len() {
                    return here();
                }
                // A map is compared by key, its order no part of its value.
                if let (Some(entries), Some(other)) = (map_entries(items), map_entries(others)) {
                    return entries.iter().find_map(|(key, a)| {
                        match other.iter().find(|(held, _)| held == key) {
                            Some((_, b)) => a.differs_at(b, tolerance, &below(path, key, false)),
                            None => Some(written(&below(path, key, false))),
                        }
                    });
                }
                let path = below(path, "layers", true);
                items
                    .iter()
                    .zip(others)
                    .enumerate()
                    .find_map(|(index, (a, b))| {
                        a.differs_at(b, tolerance, &below(&path, &index.to_string(), false))
                    })
            }
            _ => here(),
        }
    }
}

/// A list's items as a map's entries, each key as its text (a `Key`'s) or
/// else as written (an `AxisTag`'s bytes), and its value: every item a
/// pair, no key twice; `None` for any other list (a paint's layers),
/// compared in order. It reads the shape, not the field's type: every
/// `Vec<(_, _)>` the model holds is a map keyed by its first member, so a
/// list of unnamed pairs is one; a list of pairs kept in order would need
/// the registry's type at its path here.
fn map_entries(items: &[Tree]) -> Option<Vec<(String, &Tree)>> {
    let mut out: Vec<(String, &Tree)> = Vec::new();
    for item in items {
        let Tree::Tuple { name, items } = item else {
            return None;
        };
        let ([key, value], true) = (items.as_slice(), name.is_empty()) else {
            return None;
        };
        let key = key.text().map_or_else(|| key.to_string(), str::to_owned);
        if out.iter().any(|(held, _)| *held == key) {
            return None;
        }
        out.push((key, value));
    }
    (!out.is_empty()).then_some(out)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::blocks::motion::Timing;
    use crate::color::Color;
    use crate::units::Px;

    #[derive(Debug)]
    #[allow(dead_code, reason = "read through its Debug form only")]
    struct Probe {
        width: Option<Px>,
        paint: Vec<Color>,
        map: Vec<(String, Option<f32>)>,
        name: &'static str,
    }

    /// A value's tree reads through `Some`, finds a field, a map's entry
    /// and a colour, and compares numbers and colours within tolerance.
    #[test]
    fn tree_reads_debug_and_compares_within_tolerance() {
        let probe = |width: f32, red: f32| Probe {
            width: Some(Px(width)),
            paint: vec![Color {
                r: red,
                g: 0.5,
                b: 0.0,
                a: 1.0,
            }],
            map: vec![("a b".to_owned(), Some(-1.5e-7)), ("c".to_owned(), None)],
            name: "x\"y",
        };
        let tree = Tree::of(&probe(4.0, 0.25)).unwrap();
        let path = |text: &str| text.split('.').map(str::to_owned).collect::<Vec<_>>();
        assert_eq!(
            tree.at(&path("width")),
            Some(&Tree::Tuple {
                name: "Px".to_owned(),
                items: vec![Tree::Number(4.0)]
            })
        );
        assert_eq!(tree.at(&path("map.a b")), Some(&Tree::Number(-1.5e-7)));
        assert_eq!(
            tree.at(&path("map.c")),
            Some(&Tree::Word("None".to_owned()))
        );
        assert_eq!(tree.at(&path("name")), Some(&Tree::Text("x\"y".to_owned())));
        let same = Tree::of(&probe(4.0001, 0.25)).unwrap();
        assert_eq!(tree.differs(&same, &Tolerance::quote()), None);
        let wider = Tree::of(&probe(4.01, 0.25)).unwrap();
        assert_eq!(
            tree.differs(&wider, &Tolerance::quote()),
            Some("width".to_owned())
        );
        assert_eq!(tree.differs(&wider, &Tolerance::every(0.01)), None);
        assert_eq!(
            tree.differs(
                &wider,
                &Tolerance::per_leaf(vec![("width_px".to_owned(), 0.02)])
            ),
            None
        );
        let redder = Tree::of(&probe(4.0, 0.25 + 1.0 / 255.0)).unwrap();
        assert_eq!(
            tree.differs(&redder, &Tolerance::quote()),
            Some("paint.0".to_owned())
        );
        assert_eq!(tree.differs(&redder, &Tolerance::every(1.0)), None);
    }

    #[derive(Debug)]
    #[allow(dead_code, reason = "read through its Debug form only")]
    enum Shape {
        Rounded(Px),
        Plain,
    }

    #[derive(Debug)]
    #[allow(dead_code, reason = "read through its Debug form only")]
    struct Holder {
        width: Option<Px>,
        none: Option<Option<Px>>,
        color: Color,
        shape: Shape,
        map: Vec<(String, Option<f32>)>,
    }

    fn holder(width: f32) -> Holder {
        Holder {
            width: Some(Px(width)),
            none: Some(None),
            color: Color {
                r: 0.25,
                g: 0.5,
                b: 0.0,
                a: 1.0,
            },
            shape: Shape::Rounded(Px(width)),
            map: vec![("a".to_owned(), Some(2.0)), ("b".to_owned(), None)],
        }
    }

    /// `Tree::of_at` is `Tree::of` read at the path (`Tree::at`), which the
    /// gate takes it for: through `Some`, at a field, a `Some(None)`, a
    /// colour, a variant named in snake_case, a map's entry, the whole
    /// value at the empty path, and nothing where the path names nothing;
    /// and `graft` replaces the subtree at a path with another tree's there,
    /// the rest kept, a path either side lacks changing nothing.
    #[test]
    fn of_at_reads_as_at_and_graft_replaces_there() {
        let value = holder(4.0);
        let whole = Tree::of(&value).unwrap();
        for text in [
            "",
            "width",
            "none",
            "color",
            "color.r",
            "shape",
            "shape.rounded",
            "shape.plain",
            "map",
            "map.a",
            "map.b",
            "map.c",
            "missing",
        ] {
            let path: Vec<String> = text
                .split('.')
                .filter(|segment| !segment.is_empty())
                .map(str::to_owned)
                .collect();
            assert_eq!(
                Tree::of_at(&value, &path),
                whole.at(&path).cloned(),
                "{text:?}"
            );
        }
        assert!(
            whole
                .at(&["shape".to_owned(), "rounded".to_owned()])
                .is_some()
        );
        let wider = Tree::of(&holder(5.0)).unwrap();
        let width = vec!["width".to_owned()];
        let mut grafted = whole.clone();
        grafted.graft(&width, &wider);
        assert_eq!(grafted.at(&width), wider.at(&width));
        let shape = vec!["shape".to_owned()];
        assert_eq!(grafted.at(&shape), whole.at(&shape));
        let mut unchanged = whole.clone();
        unchanged.graft(&["missing".to_owned()], &wider);
        assert_eq!(unchanged, whole);
    }

    #[derive(Debug)]
    #[allow(dead_code, reason = "read through its Debug form only")]
    struct ResolvedShadowLayer {
        x: Px,
        y: Px,
    }

    #[derive(Debug)]
    #[allow(dead_code, reason = "read through its Debug form only")]
    struct Part {
        shadow: Option<Vec<ResolvedShadowLayer>>,
        timing: Timing,
        size: Option<Px>,
    }

    /// A per-leaf tolerance reaches the leaf S11 spells (S17's own
    /// `"layers.0.y_px"`): a resolved shadow's list of layers, which S11
    /// spells under `layers`, and a curve's points under its variant's
    /// name; a point size's tolerance is compared in px.
    #[test]
    fn per_leaf_tolerances_reach_the_leaves_s11_spells() {
        let part = |y: f32, x1: f32, size: f32| Part {
            shadow: Some(vec![ResolvedShadowLayer {
                x: Px(0.0),
                y: Px(y),
            }]),
            timing: Timing::CubicBezier(x1, 0.0, 0.0, 1.0),
            size: Some(Px(size)),
        };
        let path = |text: &str| vec![text.to_owned()];
        let field = |part: &Part, name: &str| Tree::of(part).unwrap().at(&path(name)).cloned();
        let (base, moved) = (part(2.0, 0.2, 16.0), part(2.4, 0.25, 16.6));
        let shadow = |part: &Part| field(part, "shadow").unwrap();
        let layers = Tolerance::per_leaf(vec![("layers.0.y_px".to_owned(), 0.5)]);
        assert_eq!(shadow(&base).differs(&shadow(&moved), &layers), None);
        assert_eq!(
            shadow(&base).differs(&shadow(&moved), &Tolerance::quote()),
            Some("0.y".to_owned())
        );
        let other = Tolerance::per_leaf(vec![("layers.0.x_px".to_owned(), 0.5)]);
        assert_eq!(
            shadow(&base).differs(&shadow(&moved), &other),
            Some("0.y".to_owned())
        );
        let timing = |part: &Part| field(part, "timing").unwrap();
        let curve = Tolerance::per_leaf(vec![("cubic_bezier.0".to_owned(), 0.1)]);
        assert_eq!(timing(&base).differs(&timing(&moved), &curve), None);
        assert_eq!(
            timing(&base).differs(&timing(&moved), &Tolerance::quote()),
            Some("0".to_owned())
        );
        // 0.5pt at 96 dpi is 0.667px: 0.6px apart holds, which 0.5px would
        // not.
        let size = |part: &Part| field(part, "size").unwrap();
        let points = Tolerance::every(0.5).in_points(&[String::new()]);
        assert_eq!(
            size(&base).differs(&size(&moved), &points.at_points_of(96.0 / 72.0)),
            None
        );
        assert_eq!(
            size(&base).differs(&size(&moved), &Tolerance::every(0.5)),
            Some(".".to_owned())
        );
        let leaf = Tolerance::per_leaf(vec![("size_pt".to_owned(), 0.5)]);
        let larger = Tree::of(&part(2.0, 0.2, 16.6)).unwrap();
        let whole = Tree::of(&base).unwrap();
        assert_eq!(
            whole.differs(&larger, &leaf.at_points_of(96.0 / 72.0)),
            None
        );
        assert_eq!(
            whole.differs(&larger, &leaf.at_points_of(1.0)),
            Some("size".to_owned())
        );
    }

    /// A measured tolerance holds at its edge and fails beyond it by more
    /// than the f32 storage slack beside it (S17 *The gate's tolerance*).
    #[test]
    fn measured_tolerance_edge_and_its_slack() {
        assert!(numbers_agree(40.5, 40.0, Some(0.5)));
        let slack = STORED * 40.5;
        assert!(slack < 1e-5);
        assert!(!numbers_agree(40.5 + 1e-5, 40.0, Some(0.5)));
        assert!(numbers_agree(40.0 + 0.5 * slack, 40.0, Some(0.0)));
        assert!(!numbers_agree(40.0 + 2.0 * slack, 40.0, Some(0.0)));
    }

    /// A measured colour compared as each renderer draws the resolved one
    /// (S17, S2.1; ADD-A131): light teal's standalone red, -0.17 at alpha
    /// 0.5, draws as 0 premultiplied and clamped; a channel 1.2 clamped
    /// straight draws 1; where no source states the drawing it fails out
    /// of the gamut; the quote check's tolerance, and a measured value read
    /// from an API rather than from drawn pixels, draw nothing.
    #[test]
    fn measured_colour_drawn_as_its_renderer_draws_it() {
        let colour = |r: f64, a: f64| Tree::Color([r, 0.5, 0.5, a]);
        let read = Tolerance::every(1.0);
        let measured = read.clone().from_drawn_pixels();
        let gtk = measured.drawn_by(Drawing::Premultiplied);
        assert_eq!(colour(-0.17, 0.5).differs(&colour(0.0, 0.5), &gtk), None);
        assert!(
            colour(-0.17, 0.5)
                .differs(&colour(0.0, 0.5), &measured)
                .is_some()
        );
        assert_eq!(colour(0.3, 0.0).differs(&colour(0.9, 0.0), &gtk), None);
        let compose = measured.drawn_by(Drawing::Clamped);
        assert_eq!(colour(1.2, 0.5).differs(&colour(1.0, 0.5), &compose), None);
        assert!(colour(1.2, 0.5).differs(&colour(1.0, 0.5), &gtk).is_some());
        let unread = measured.drawn_by(Drawing::Unread);
        assert_eq!(colour(0.2, 0.5).differs(&colour(0.2, 0.5), &unread), None);
        assert_eq!(
            colour(-0.17, 0.5).differs(&colour(0.0, 0.5), &unread),
            Some(
                ". (out of the gamut, where no source states how the renderer draws it)".to_owned()
            )
        );
        let as_read = read.drawn_by(Drawing::Premultiplied);
        assert!(
            colour(-0.17, 0.5)
                .differs(&colour(0.0, 0.5), &as_read)
                .is_some()
        );
        assert_eq!(
            colour(-0.17, 0.5).differs(&colour(-0.17, 0.5), &read.drawn_by(Drawing::Unread)),
            None
        );
        let quoted = Tolerance::quote().drawn_by(Drawing::Premultiplied);
        assert!(
            colour(-0.17, 0.5)
                .differs(&colour(0.0, 0.5), &quoted)
                .is_some()
        );
    }
}
