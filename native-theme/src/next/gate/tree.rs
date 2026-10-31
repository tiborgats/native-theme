//! A resolved value as the gate compares it (S17): read from the value's
//! `Debug` form, which every resolved type derives, so one comparison
//! serves every type the model holds, its numeric and colour leaves within
//! a tolerance and every other leaf exactly.

use std::fmt;

/// A resolved value's tree: a struct's fields by name, a tuple's or a
/// list's items in order, and its leaves. `Some(x)` reads as `x`, so a
/// value and its absence differ by a leaf `None`.
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

/// One token of a `Debug` form.
#[derive(Clone, Debug, PartialEq)]
enum Token {
    Ident(String),
    Number(f64),
    Text(String),
    Punct(char),
}

/// The tokens of `text`, or `None` where it holds what no derived `Debug`
/// writes.
fn tokens(text: &str) -> Option<Vec<Token>> {
    let mut out = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '{' | '}' | '(' | ')' | '[' | ']' | ',' | ':' => out.push(Token::Punct(c)),
            '"' => {
                let mut value = String::new();
                loop {
                    let (_, c) = chars.next()?;
                    match c {
                        '"' => break,
                        '\\' => {
                            let (_, escaped) = chars.next()?;
                            value.push(match escaped {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '0' => '\0',
                                other => other,
                            });
                        }
                        other => value.push(other),
                    }
                }
                out.push(Token::Text(value));
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
                    _ => Token::Ident(word.to_owned()),
                });
            }
            _ => return None,
        }
    }
    Some(out)
}

/// A reader over a `Debug` form's tokens.
struct Parser {
    tokens: Vec<Token>,
    at: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at)
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.at).cloned();
        self.at = self.at.saturating_add(1);
        token
    }

    fn punct(&mut self, c: char) -> Option<()> {
        (self.next()? == Token::Punct(c)).then_some(())
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
        match self.next()? {
            Token::Number(x) => Some(Tree::Number(x)),
            Token::Text(text) => Some(Tree::Text(text)),
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
                        let Token::Ident(field) = self.next()? else {
                            return None;
                        };
                        self.punct(':')?;
                        fields.push((field, self.value()?));
                        if !self.at_punct('}') {
                            self.punct(',')?;
                        }
                    }
                    self.punct('}')?;
                    Some(fields_tree(name, fields))
                } else if self.at_punct('(') {
                    self.punct('(')?;
                    let mut items = self.items(')')?;
                    if name == "Some" && items.len() == 1 {
                        return items.pop();
                    }
                    Some(Tree::Tuple { name, items })
                } else {
                    Some(Tree::Word(name))
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
        let mut parser = Parser {
            tokens: tokens(&format!("{value:?}"))?,
            at: 0,
        };
        let tree = parser.value()?;
        parser.peek().is_none().then_some(tree)
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
                let limit = tolerance.at(path).unwrap_or(COLOR_PRECISION);
                let close = a
                    .iter()
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::next::blocks::motion::Timing;
    use crate::next::color::Color;
    use crate::next::units::Px;

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
}
