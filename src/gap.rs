use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{
    LengthPercentageCalculation, NonNegativeLengthPercentage, NonNegativeLengthPercentageValue,
    serialization::CssWriteTo,
};

/// One component of the `gap`, `row-gap`, or `column-gap` grammar.
///
/// [CSS Box Alignment gap grammar]: https://www.w3.org/TR/css-align-3/#column-row-gap
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum GapValue {
    /// The `normal` keyword.
    Normal,
    /// A non-negative length or percentage.
    LengthPercentage(NonNegativeLengthPercentageValue),
}

impl From<NonNegativeLengthPercentage> for GapValue {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::LengthPercentage(value.into())
    }
}

impl From<LengthPercentageCalculation> for GapValue {
    fn from(value: LengthPercentageCalculation) -> Self {
        Self::LengthPercentage(value.into())
    }
}

impl CssWriteTo for GapValue {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Normal => w.write_str("normal"),
            Self::LengthPercentage(value) => value.css_fmt(w),
        }
    }
}

/// A checked `gap` shorthand value.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Gap {
    /// One value for both row and column gaps.
    One(GapValue),
    /// Separate row and column gaps.
    RowColumn(GapValue, GapValue),
}

impl CssWriteTo for Gap {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::One(value) => value.css_fmt(w),
            Self::RowColumn(row, column) => {
                row.css_fmt(w)?;
                w.write_char(' ')?;
                column.css_fmt(w)
            }
        }
    }
}

impl_checked_value!(GapValue, Gap);
