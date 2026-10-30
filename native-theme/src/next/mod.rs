//! The 0.8.0 model, built beside today's until it replaces it.
//!
//! Its re-exports are the shape the crate root takes when this module moves
//! into place: the units, keys, colours and values every block is made of.

pub mod color;
mod error;
mod expr;
mod key;
mod roles;
mod selector;
mod units;
mod value;

pub use color::Color;
pub use error::KeyError;
pub use expr::Expr;
pub use key::Key;
pub use roles::SurfaceSet;
pub use selector::Scheme;
pub use units::{
    Axes, Deg, FontSize, Fraction, Length, Ms, Point, Px, Rect, Scalar, Sides, Size, ViewBox,
};
pub use value::{SchemeMap, SurfaceMap, Value};
