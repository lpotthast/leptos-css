use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::serialization::CssWriteTo;

/// A typed `z-index` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ZIndex {
    /// The `auto` keyword.
    Auto,
    /// An integer stack level.
    Integer(i32),
}

impl CssWriteTo for ZIndex {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::Integer(value) => write!(w, "{value}"),
        }
    }
}

impl_checked_value!(ZIndex);
