//! The expression grammar (S2.2), read by recursive descent.

use std::fmt;

use super::{ElementDatum, Expr, Func, Literal, MAX_DEPTH, Name, Ref};
use crate::next::color::Color;
use crate::next::key::Key;
use crate::next::roles::SurfaceSet;
use crate::next::units::{Deg, Ms, Px};

/// The words that are not names (S2.2): each is a reference, a literal form
/// or `active(…)`.
const RESERVED: [&str; 11] = [
    "base",
    "scheme",
    "font_size",
    "line_height",
    "body_font_height",
    "text_scale",
    "active",
    "srgb",
    "display_p3",
    "true",
    "false",
];

/// Why a text is not an expression, and where.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ParseError {
    /// The byte offset in the text where the problem was found.
    pub offset: usize,
    /// The problem.
    pub kind: ParseErrorKind,
}

/// What is wrong with an expression's text.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    /// The text ends where the grammar needs more.
    UnexpectedEnd,
    /// A character the grammar does not allow here.
    Unexpected(char),
    /// Text after a whole expression.
    TrailingInput,
    /// A number not of the form `["-"] digits ["." digits]`.
    BadNumber,
    /// A number too large to be finite.
    NonFinite,
    /// A unit other than `px`, `ms`, `deg` and `pt` after a number.
    UnknownUnit(String),
    /// A colour literal that is not one of S2.1's forms.
    BadColor(String),
    /// A path segment or key outside `[a-z0-9][a-z0-9_]*`.
    BadSegment(String),
    /// A reserved word used as a name or a function.
    ReservedWord(String),
    /// A call of a name no function has.
    UnknownFunction(String),
    /// A word followed by `.` that starts no reference.
    UnknownReference(String),
    /// `role.<set>` naming no surface set.
    UnknownSurfaceSet(String),
    /// `element.<data>` naming no element datum.
    UnknownElementData(String),
    /// `active(…)` around something other than a reference.
    NotAReference,
    /// Calls nested deeper than [`MAX_DEPTH`].
    TooDeep,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at byte {}: ", self.offset)?;
        match &self.kind {
            ParseErrorKind::UnexpectedEnd => f.write_str("the expression ends too early"),
            ParseErrorKind::Unexpected(c) => write!(f, "unexpected {c:?}"),
            ParseErrorKind::TrailingInput => f.write_str("text after the expression"),
            ParseErrorKind::BadNumber => f.write_str("a number is `[\"-\"] digits [\".\" digits]`"),
            ParseErrorKind::NonFinite => f.write_str("the number is not finite"),
            ParseErrorKind::UnknownUnit(unit) => {
                write!(f, "unknown unit {unit:?}: `px`, `ms`, `deg` or `pt`")
            }
            ParseErrorKind::BadColor(text) => write!(f, "{text:?} is not a colour literal"),
            ParseErrorKind::BadSegment(text) => {
                write!(f, "{text:?} is not a name, `[a-z0-9][a-z0-9_]*`")
            }
            ParseErrorKind::ReservedWord(word) => write!(f, "{word:?} is a reserved word"),
            ParseErrorKind::UnknownFunction(name) => write!(f, "no function is named {name:?}"),
            ParseErrorKind::UnknownReference(word) => {
                write!(f, "{word:?} starts no reference")
            }
            ParseErrorKind::UnknownSurfaceSet(name) => {
                write!(f, "{name:?} is not a surface set")
            }
            ParseErrorKind::UnknownElementData(name) => {
                write!(f, "{name:?} is not element data")
            }
            ParseErrorKind::NotAReference => f.write_str("`active(…)` takes a reference"),
            ParseErrorKind::TooDeep => write!(f, "nested deeper than {MAX_DEPTH}"),
        }
    }
}

impl std::error::Error for ParseError {}

impl Expr {
    /// The expression `text` states: the text after a TOML value's `=`.
    /// Whitespace between the grammar's tokens is insignificant; a literal, a
    /// name and a path segment are each written without it.
    ///
    /// ```
    /// use native_theme::next::expr::{Expr, Func};
    ///
    /// let expr = Expr::parse("alpha(surface.foreground, 0.5)")?;
    /// assert!(matches!(expr, Expr::Call(Func::Alpha, _)));
    /// # Ok::<(), native_theme::next::expr::ParseError>(())
    /// ```
    pub fn parse(text: &str) -> Result<Expr, ParseError> {
        let mut parser = Parser { text, offset: 0 };
        let expr = parser.expr(0)?;
        parser.skip_whitespace();
        match parser.peek() {
            None => Ok(expr),
            Some(_) => Err(parser.error(ParseErrorKind::TrailingInput)),
        }
    }
}

struct Parser<'a> {
    text: &'a str,
    offset: usize,
}

impl<'a> Parser<'a> {
    fn error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError {
            offset: self.offset,
            kind,
        }
    }

    fn peek(&self) -> Option<char> {
        self.text.get(self.offset..)?.chars().next()
    }

    fn bump(&mut self, c: char) {
        self.offset = self.offset.saturating_add(c.len_utf8());
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek().filter(|c| c.is_whitespace()) {
            self.bump(c);
        }
    }

    /// Consumes `expected` after any whitespace, or fails on what is there.
    fn consume(&mut self, expected: char) -> Result<(), ParseError> {
        self.skip_whitespace();
        match self.peek() {
            Some(c) if c == expected => {
                self.bump(c);
                Ok(())
            }
            Some(c) => Err(self.error(ParseErrorKind::Unexpected(c))),
            None => Err(self.error(ParseErrorKind::UnexpectedEnd)),
        }
    }

    /// The longest run of characters that `keep` accepts, from here.
    fn take_while(&mut self, keep: impl Fn(char) -> bool) -> &'a str {
        let start = self.offset;
        while let Some(c) = self.peek().filter(|c| keep(*c)) {
            self.bump(c);
        }
        self.text.get(start..self.offset).unwrap_or_default()
    }

    fn word(&mut self) -> String {
        self.take_while(|c| c.is_ascii_alphanumeric() || c == '_')
            .to_owned()
    }

    /// One key or path segment, `[a-z0-9][a-z0-9_]*`.
    fn segment(&mut self) -> Result<Key, ParseError> {
        self.skip_whitespace();
        let start = self.offset;
        let word = self.word();
        if word.is_empty() {
            return Err(match self.peek() {
                Some(c) => self.error(ParseErrorKind::Unexpected(c)),
                None => self.error(ParseErrorKind::UnexpectedEnd),
            });
        }
        Key::new(&word).map_err(|_| ParseError {
            offset: start,
            kind: ParseErrorKind::BadSegment(word),
        })
    }

    /// `"." segment { "." segment }`: a path of at least one segment.
    fn path(&mut self) -> Result<Vec<Key>, ParseError> {
        self.consume('.')?;
        let mut path = vec![self.segment()?];
        loop {
            let before = self.offset;
            self.skip_whitespace();
            if self.peek() == Some('.') {
                self.bump('.');
                path.push(self.segment()?);
            } else {
                self.offset = before;
                return Ok(path);
            }
        }
    }

    /// `expr` at nesting `depth`: the calls and `active(…)` around it.
    fn expr(&mut self, depth: usize) -> Result<Expr, ParseError> {
        self.skip_whitespace();
        match self.peek() {
            None => Err(self.error(ParseErrorKind::UnexpectedEnd)),
            Some('#') => self.hex_color(),
            Some('@') => {
                self.bump('@');
                Ok(Expr::Lit(Literal::Name(Name::Key(self.key_after_at()?))))
            }
            Some(c) if c == '-' || c.is_ascii_digit() => self.number(),
            Some(c) if c.is_ascii_lowercase() => self.word_expr(depth),
            Some(c) => Err(self.error(ParseErrorKind::Unexpected(c))),
        }
    }

    /// The key right after an `@`, with no whitespace between.
    fn key_after_at(&mut self) -> Result<Key, ParseError> {
        match self.peek() {
            Some(c) if c.is_whitespace() => Err(self.error(ParseErrorKind::Unexpected(c))),
            _ => self.segment(),
        }
    }

    fn hex_color(&mut self) -> Result<Expr, ParseError> {
        let start = self.offset;
        self.bump('#');
        let digits = self.take_while(|c| c.is_ascii_alphanumeric()).to_owned();
        let text = format!("#{digits}");
        Color::parse(&text)
            .map(|c| Expr::Lit(Literal::Color(c)))
            .ok_or(ParseError {
                offset: start,
                kind: ParseErrorKind::BadColor(text),
            })
    }

    /// `["-"] digits ["." digits]` and an optional unit right after it.
    fn number(&mut self) -> Result<Expr, ParseError> {
        let start = self.offset;
        let negative = self.peek() == Some('-');
        if negative {
            self.bump('-');
        }
        let whole = self.take_while(|c| c.is_ascii_digit()).len();
        let fraction = if self.peek() == Some('.') {
            self.bump('.');
            Some(self.take_while(|c| c.is_ascii_digit()).len())
        } else {
            None
        };
        if whole == 0 || fraction == Some(0) {
            return Err(ParseError {
                offset: start,
                kind: ParseErrorKind::BadNumber,
            });
        }
        let digits = self.text.get(start..self.offset).unwrap_or_default();
        let value = match digits.parse::<f32>() {
            Ok(value) if value.is_finite() => value,
            _ => {
                return Err(ParseError {
                    offset: start,
                    kind: ParseErrorKind::NonFinite,
                });
            }
        };
        let unit_start = self.offset;
        let unit = self.word();
        let literal = match unit.as_str() {
            "" => Literal::Number(value),
            "px" => Literal::Px(Px(value)),
            "ms" => Literal::Ms(Ms(value)),
            "deg" => Literal::Deg(Deg(value)),
            "pt" => Literal::Pt(value),
            _ => {
                return Err(ParseError {
                    offset: unit_start,
                    kind: ParseErrorKind::UnknownUnit(unit),
                });
            }
        };
        Ok(Expr::Lit(literal))
    }

    /// An expression starting with a word: a call, a reference, a colour
    /// literal's function form, a reserved word or a name.
    fn word_expr(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let start = self.offset;
        let word = self.word();
        let after_word = self.offset;
        self.skip_whitespace();
        let refused = |kind| ParseError {
            offset: start,
            kind,
        };
        match self.peek() {
            Some('(') => match word.as_str() {
                "srgb" | "display_p3" => self.color_function(start, &word),
                "active" => {
                    let depth = self.deeper(depth)?;
                    self.bump('(');
                    let inner = self.expr(depth)?;
                    self.consume(')')?;
                    match inner {
                        Expr::Ref(inner) => Ok(Expr::Ref(Ref::Active(Box::new(inner)))),
                        _ => Err(refused(ParseErrorKind::NotAReference)),
                    }
                }
                _ if RESERVED.contains(&word.as_str()) => {
                    Err(refused(ParseErrorKind::ReservedWord(word)))
                }
                _ => {
                    let func = Func::from_name(&word)
                        .ok_or_else(|| refused(ParseErrorKind::UnknownFunction(word.clone())))?;
                    let depth = self.deeper(depth)?;
                    self.bump('(');
                    let mut args = vec![self.expr(depth)?];
                    loop {
                        self.skip_whitespace();
                        match self.peek() {
                            Some(',') => {
                                self.bump(',');
                                args.push(self.expr(depth)?);
                            }
                            _ => {
                                self.consume(')')?;
                                return Ok(Expr::Call(func, args));
                            }
                        }
                    }
                }
            },
            Some('.') => {
                self.offset = after_word;
                self.reference(start, &word).map(Expr::Ref)
            }
            _ => {
                self.offset = after_word;
                let reference = match word.as_str() {
                    "base" => Ref::Base,
                    "scheme" => Ref::Scheme,
                    "font_size" => Ref::FontSize,
                    "line_height" => Ref::LineHeight,
                    "body_font_height" => Ref::BodyFontHeight,
                    "text_scale" => Ref::TextScale,
                    "true" => return Ok(Expr::Lit(Literal::Bool(true))),
                    "false" => return Ok(Expr::Lit(Literal::Bool(false))),
                    "active" | "srgb" | "display_p3" => {
                        return Err(refused(ParseErrorKind::ReservedWord(word)));
                    }
                    _ => {
                        let name = Key::new(&word)
                            .map_err(|_| refused(ParseErrorKind::BadSegment(word.clone())))?;
                        return Ok(Expr::Lit(Literal::Name(Name::Variant(name))));
                    }
                };
                Ok(Expr::Ref(reference))
            }
        }
    }

    /// The nesting depth inside one more call, if it is allowed.
    fn deeper(&self, depth: usize) -> Result<usize, ParseError> {
        depth
            .checked_add(1)
            .filter(|depth| *depth <= MAX_DEPTH)
            .ok_or_else(|| self.error(ParseErrorKind::TooDeep))
    }

    /// `srgb(…)` or `display_p3(…)`, its arguments number literals.
    fn color_function(&mut self, start: usize, name: &str) -> Result<Expr, ParseError> {
        self.bump('(');
        let arguments = self.take_while(|c| c != ')' && c != '(').to_owned();
        self.consume(')')?;
        let text = format!("{name}({arguments})");
        Color::parse(&text)
            .map(|c| Expr::Lit(Literal::Color(c)))
            .ok_or(ParseError {
                offset: start,
                kind: ParseErrorKind::BadColor(text),
            })
    }

    /// A reference whose first word, `prefix`, a `.` follows.
    fn reference(&mut self, start: usize, prefix: &str) -> Result<Ref, ParseError> {
        match prefix {
            "role" => {
                self.consume('.')?;
                self.skip_whitespace();
                if self.peek() == Some('@') {
                    self.bump('@');
                    return Ok(Ref::KeyedRole(self.key_after_at()?));
                }
                let set_start = self.offset;
                let set = self.segment()?;
                if set.as_str() == "global" {
                    return Ok(Ref::GlobalRole(self.path()?));
                }
                let set = SurfaceSet::from_name(set.as_str()).ok_or_else(|| ParseError {
                    offset: set_start,
                    kind: ParseErrorKind::UnknownSurfaceSet(set.to_string()),
                })?;
                Ok(Ref::Role {
                    set,
                    path: self.path()?,
                })
            }
            "surface" => Ok(Ref::Surface(self.path()?)),
            "token" => Ok(Ref::Token(self.path()?)),
            "input" => Ok(Ref::Input(self.path()?)),
            "part" => Ok(Ref::Part(self.path()?)),
            "element" => {
                self.consume('.')?;
                self.skip_whitespace();
                let name_start = self.offset;
                let name = self.segment()?;
                ElementDatum::from_name(name.as_str())
                    .map(Ref::Element)
                    .ok_or(ParseError {
                        offset: name_start,
                        kind: ParseErrorKind::UnknownElementData(name.to_string()),
                    })
            }
            "widget" => {
                self.consume('.')?;
                let part = self.segment()?;
                Ok(Ref::Widget {
                    part,
                    path: self.path()?,
                })
            }
            "values" => {
                self.consume('.')?;
                Ok(Ref::Values(self.segment()?))
            }
            _ => Err(ParseError {
                offset: start,
                kind: ParseErrorKind::UnknownReference(prefix.to_owned()),
            }),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::key;

    fn parse(text: &str) -> Expr {
        Expr::parse(text).unwrap_or_else(|e| panic!("{text:?}: {e}"))
    }

    fn refused(text: &str) -> ParseErrorKind {
        Expr::parse(text)
            .map(|e| panic!("{text:?} parsed as {e:?}"))
            .unwrap_err()
            .kind
    }

    fn lit(literal: Literal) -> Expr {
        Expr::Lit(literal)
    }

    fn reference(r: Ref) -> Expr {
        Expr::Ref(r)
    }

    fn rgba(r: f32, g: f32, b: f32, a: f32) -> Literal {
        Literal::Color(Color { r, g, b, a })
    }

    // expr := literal | ref | call, one test per production below.

    #[test]
    fn parses_colour_literals() {
        assert_eq!(parse("#ff0000"), lit(rgba(1.0, 0.0, 0.0, 1.0)));
        assert_eq!(parse("#00ff0080"), lit(rgba(0.0, 1.0, 0.0, 128.0 / 255.0)));
        assert_eq!(parse("srgb(0.25, 0.5, 1)"), lit(rgba(0.25, 0.5, 1.0, 1.0)));
        assert_eq!(
            parse("srgb ( 0.25 , 0.5 , 1 , 0.5 )"),
            lit(rgba(0.25, 0.5, 1.0, 0.5))
        );
        let p3 = Color::parse("display_p3(1, 0, 0)").unwrap();
        assert_eq!(parse("display_p3(1, 0, 0)"), lit(Literal::Color(p3)));
        assert!(matches!(refused("#ff00"), ParseErrorKind::BadColor(_)));
        assert!(matches!(
            refused("srgb(0.5, 0.5)"),
            ParseErrorKind::BadColor(_)
        ));
        assert!(matches!(
            refused("srgb(token.a, 0, 0)"),
            ParseErrorKind::BadColor(_)
        ));
    }

    #[test]
    fn parses_number() {
        assert_eq!(parse("4"), lit(Literal::Number(4.0)));
        assert_eq!(parse("0.25"), lit(Literal::Number(0.25)));
        assert_eq!(refused(".5"), ParseErrorKind::Unexpected('.'));
        assert_eq!(refused("1."), ParseErrorKind::BadNumber);
        assert_eq!(refused("-"), ParseErrorKind::BadNumber);
        assert_eq!(refused(&"9".repeat(60)), ParseErrorKind::NonFinite);
    }

    #[test]
    fn parses_length_duration_angle_font_size() {
        assert_eq!(parse("4px"), lit(Literal::Px(Px(4.0))));
        assert_eq!(parse("150ms"), lit(Literal::Ms(Ms(150.0))));
        assert_eq!(parse("90deg"), lit(Literal::Deg(Deg(90.0))));
        assert_eq!(parse("10.5pt"), lit(Literal::Pt(10.5)));
        assert_eq!(refused("4em"), ParseErrorKind::UnknownUnit("em".to_owned()));
        assert_eq!(refused("4 px"), ParseErrorKind::TrailingInput);
    }

    #[test]
    fn parses_negative_literal() {
        assert_eq!(parse("-1px"), lit(Literal::Px(Px(-1.0))));
        assert_eq!(parse("-0.5"), lit(Literal::Number(-0.5)));
        assert_eq!(refused("-token.a"), ParseErrorKind::BadNumber);
    }

    #[test]
    fn parses_name_variant() {
        assert_eq!(
            parse("inactive"),
            lit(Literal::Name(Name::Variant(key!("inactive"))))
        );
        assert_eq!(refused("Inactive"), ParseErrorKind::Unexpected('I'));
    }

    #[test]
    fn parses_name_key() {
        assert_eq!(
            parse("@primary_container"),
            lit(Literal::Name(Name::Key(key!("primary_container"))))
        );
        assert_eq!(refused("@ primary"), ParseErrorKind::Unexpected(' '));
        assert_eq!(
            refused("@_primary"),
            ParseErrorKind::BadSegment("_primary".to_owned())
        );
    }

    #[test]
    fn parses_bool() {
        assert_eq!(parse("true"), lit(Literal::Bool(true)));
        assert_eq!(parse("false"), lit(Literal::Bool(false)));
    }

    #[test]
    fn parses_path_segments() {
        assert_eq!(
            parse("token.elevation.1"),
            reference(Ref::Token(vec![key!("elevation"), key!("1")]))
        );
        assert_eq!(
            refused("token.a._b"),
            ParseErrorKind::BadSegment("_b".to_owned())
        );
        assert_eq!(refused("token."), ParseErrorKind::UnexpectedEnd);
        assert_eq!(
            refused("token.A"),
            ParseErrorKind::BadSegment("A".to_owned())
        );
    }

    #[test]
    fn parses_role_set_path() {
        assert_eq!(
            parse("role.selection.disabled.foreground"),
            reference(Ref::Role {
                set: SurfaceSet::Selection,
                path: vec![key!("disabled"), key!("foreground")],
            })
        );
        assert_eq!(
            parse("role.secondary_sidebar.background"),
            reference(Ref::Role {
                set: SurfaceSet::SecondarySidebar,
                path: vec![key!("background")],
            })
        );
        assert_eq!(
            refused("role.sheet.background"),
            ParseErrorKind::UnknownSurfaceSet("sheet".to_owned())
        );
        assert_eq!(refused("role.window"), ParseErrorKind::UnexpectedEnd);
    }

    #[test]
    fn parses_role_global() {
        assert_eq!(
            parse("role.global.status.error.fill"),
            reference(Ref::GlobalRole(vec![
                key!("status"),
                key!("error"),
                key!("fill")
            ]))
        );
    }

    #[test]
    fn parses_role_keyed() {
        assert_eq!(
            parse("role.@shade_light"),
            reference(Ref::KeyedRole(key!("shade_light")))
        );
    }

    #[test]
    fn parses_surface() {
        assert_eq!(
            parse("surface.foreground"),
            reference(Ref::Surface(vec![key!("foreground")]))
        );
    }

    #[test]
    fn parses_active() {
        assert_eq!(
            parse("active(role.view.background)"),
            reference(Ref::Active(Box::new(Ref::Role {
                set: SurfaceSet::View,
                path: vec![key!("background")],
            })))
        );
        assert_eq!(
            parse("active( surface.background )"),
            reference(Ref::Active(Box::new(Ref::Surface(vec![key!(
                "background"
            )]))))
        );
        assert_eq!(refused("active(4px)"), ParseErrorKind::NotAReference);
    }

    #[test]
    fn parses_token() {
        assert_eq!(
            parse("token.behavior.focus_visible_when"),
            reference(Ref::Token(vec![
                key!("behavior"),
                key!("focus_visible_when")
            ]))
        );
    }

    #[test]
    fn parses_input() {
        assert_eq!(
            parse("input.accent_ramp.light1"),
            reference(Ref::Input(vec![key!("accent_ramp"), key!("light1")]))
        );
    }

    #[test]
    fn parses_base_and_scheme() {
        assert_eq!(parse("base"), reference(Ref::Base));
        assert_eq!(parse("scheme"), reference(Ref::Scheme));
    }

    #[test]
    fn parses_font_size_and_line_height() {
        assert_eq!(parse("font_size"), reference(Ref::FontSize));
        assert_eq!(parse("line_height"), reference(Ref::LineHeight));
    }

    #[test]
    fn parses_element_data() {
        for datum in ElementDatum::ALL {
            assert_eq!(
                parse(&format!("element.{}", datum.name())),
                reference(Ref::Element(datum))
            );
        }
        assert_eq!(
            refused("element.depth"),
            ParseErrorKind::UnknownElementData("depth".to_owned())
        );
        assert_eq!(refused("element.width.x"), ParseErrorKind::TrailingInput);
    }

    #[test]
    fn parses_body_font_height() {
        assert_eq!(parse("body_font_height"), reference(Ref::BodyFontHeight));
    }

    #[test]
    fn parses_part() {
        assert_eq!(
            parse("part.corners.top_left"),
            reference(Ref::Part(vec![key!("corners"), key!("top_left")]))
        );
    }

    #[test]
    fn parses_widget() {
        assert_eq!(
            parse("widget.frame.shadow"),
            reference(Ref::Widget {
                part: key!("frame"),
                path: vec![key!("shadow")],
            })
        );
        assert_eq!(
            parse("widget.label.char_width"),
            reference(Ref::Widget {
                part: key!("label"),
                path: vec![key!("char_width")],
            })
        );
        assert_eq!(refused("widget.frame"), ParseErrorKind::UnexpectedEnd);
    }

    #[test]
    fn parses_values() {
        assert_eq!(
            parse("values.stroke_weight"),
            reference(Ref::Values(key!("stroke_weight")))
        );
        assert_eq!(refused("values.a.b"), ParseErrorKind::TrailingInput);
    }

    #[test]
    fn parses_text_scale() {
        assert_eq!(parse("text_scale"), reference(Ref::TextScale));
    }

    #[test]
    fn parses_call() {
        assert_eq!(
            parse("mix(surface.background, #000000, 0.5)"),
            Expr::Call(
                Func::Mix,
                vec![
                    reference(Ref::Surface(vec![key!("background")])),
                    lit(rgba(0.0, 0.0, 0.0, 1.0)),
                    lit(Literal::Number(0.5)),
                ]
            )
        );
        assert_eq!(
            parse("floor(add(1px, 2px))"),
            Expr::Call(
                Func::Floor,
                vec![Expr::Call(
                    Func::Add,
                    vec![lit(Literal::Px(Px(1.0))), lit(Literal::Px(Px(2.0)))]
                )]
            )
        );
        assert_eq!(refused("mix()"), ParseErrorKind::Unexpected(')'));
        assert_eq!(refused("mix(1,)"), ParseErrorKind::Unexpected(')'));
        assert_eq!(refused("mix(1"), ParseErrorKind::UnexpectedEnd);
        assert_eq!(
            refused("lighten(#000000, 0.5)"),
            ParseErrorKind::UnknownFunction("lighten".to_owned())
        );
        assert_eq!(
            refused("tokens.a"),
            ParseErrorKind::UnknownReference("tokens".to_owned())
        );
    }

    #[test]
    fn whitespace_is_insignificant() {
        assert_eq!(
            parse(" alpha ( token . a . b ,\t0.5 ) "),
            parse("alpha(token.a.b,0.5)")
        );
        assert_eq!(parse(" role . @x "), parse("role.@x"));
    }

    #[test]
    fn reserved_words_refused() {
        for word in ["active", "srgb", "display_p3"] {
            assert_eq!(
                refused(word),
                ParseErrorKind::ReservedWord(word.to_owned()),
                "{word}"
            );
        }
        for word in [
            "base",
            "scheme",
            "font_size",
            "line_height",
            "body_font_height",
            "text_scale",
            "true",
            "false",
        ] {
            assert_eq!(
                refused(&format!("{word}(1)")),
                ParseErrorKind::ReservedWord(word.to_owned()),
                "{word}"
            );
            assert!(
                !matches!(parse(word), Expr::Lit(Literal::Name(_))),
                "{word} is not a name"
            );
        }
        for word in RESERVED {
            assert_eq!(Func::from_name(word), None, "{word}");
        }
        assert_eq!(
            refused("mix(scheme(1), #000000, 0.5)"),
            ParseErrorKind::ReservedWord("scheme".to_owned())
        );
        assert_eq!(
            refused("base.x"),
            ParseErrorKind::UnknownReference("base".to_owned())
        );
    }

    #[test]
    fn depth_32_parses() {
        let text = format!("{}1px{}", "floor(".repeat(MAX_DEPTH), ")".repeat(MAX_DEPTH));
        assert!(Expr::parse(&text).is_ok());
        let active = format!(
            "{}role.global.accent{}",
            "active(".repeat(MAX_DEPTH),
            ")".repeat(MAX_DEPTH)
        );
        assert!(Expr::parse(&active).is_ok());
    }

    #[test]
    fn depth_33_refused() {
        let depth = MAX_DEPTH + 1;
        let text = format!("{}1px{}", "floor(".repeat(depth), ")".repeat(depth));
        assert_eq!(refused(&text), ParseErrorKind::TooDeep);
        let mixed = format!(
            "{}active(role.global.accent){}",
            "floor(".repeat(MAX_DEPTH),
            ")".repeat(MAX_DEPTH)
        );
        assert_eq!(refused(&mixed), ParseErrorKind::TooDeep);
        let deep = format!("{}1px", "floor(".repeat(100_000));
        assert_eq!(refused(&deep), ParseErrorKind::TooDeep);
    }

    #[test]
    fn display_round_trips() {
        for text in [
            "#ff0000",
            "srgb(0.25, 0.5, 1, 0.5)",
            "-1.5px",
            "150ms",
            "90deg",
            "10.5pt",
            "inactive",
            "@primary",
            "true",
            "role.window.disabled.foreground",
            "role.global.accent",
            "role.@shade_light",
            "surface.foreground",
            "active(role.view.background)",
            "token.elevation.1",
            "input.accent",
            "base",
            "scheme",
            "font_size",
            "line_height",
            "element.max_char_width",
            "body_font_height",
            "part.corners.top_left",
            "widget.frame.shadow",
            "values.delay",
            "text_scale",
            "pick(element.item_index, #ff0000, #00ff00)",
        ] {
            let expr = parse(text);
            assert_eq!(parse(&expr.to_string()), expr, "{text}");
        }
    }
}
