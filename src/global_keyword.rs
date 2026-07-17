use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::serialization::CssWriteTo;

/// CSS-wide keywords accepted by ordinary CSS properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GlobalKeyword {
    /// The `inherit` keyword.
    Inherit,
    /// The `initial` keyword.
    Initial,
    /// The `unset` keyword.
    Unset,
    /// The `revert` keyword.
    Revert,
    /// The `revert-layer` keyword.
    RevertLayer,
}

impl CssWriteTo for GlobalKeyword {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Unset => "unset",
            Self::Revert => "revert",
            Self::RevertLayer => "revert-layer",
        })
    }
}

impl_checked_value!(GlobalKeyword);
