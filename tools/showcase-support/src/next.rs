//! The shared code's reads of the 0.8.0 model, `native_theme::next`, beside
//! the 0.7.2 ones until the showcases are ported (docs/v0.8.0-plan.md, M4,
//! "What is kept twice" and step 2): each module here is its namesake's
//! counterpart and takes what the 0.7.2 one does from the 0.8.0
//! `ResolvedTheme`, its serialised form or its `ThemeContext`. What reads no
//! model (the family labels, the pickers' rows, the element list) has no
//! counterpart and is used from its own module.
//!
//! - [`values`]: a leaf's value, read from the serialised form;
//! - [`info`]: a listed element's Widget Info rows;
//! - [`rows`]: a table row's height under the text scale;
//! - [`status`]: the status bar's font and accessibility items.

pub mod info;
pub mod rows;
pub mod status;
pub mod values;
