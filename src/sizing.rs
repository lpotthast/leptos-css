use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{
    CssDimension, CssDimensionExpr, LengthPercentageCalculation, NonNegativeLengthPercentage,
    NonNegativeLengthPercentageValue, serialization::CssWriteTo,
};

/// Values supported by the typed `width`, `height`, and minimum-size property APIs.
///
/// The `stretch`, bare `fit-content`, and `contain` keywords extend the CSS Sizing Level 3
/// grammar as defined by CSS Sizing Level 4.
///
/// [CSS Sizing Level 3]: https://www.w3.org/TR/css-sizing-3/#sizing-values
/// [CSS Sizing Level 4]: https://www.w3.org/TR/css-sizing-4/#sizing-values
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Size {
    /// The `auto` keyword.
    Auto,
    /// A non-negative length or percentage.
    LengthPercentage(NonNegativeLengthPercentage),
    /// A typed length-percentage expression.
    Calculation(LengthPercentageCalculation),
    /// The `min-content` keyword.
    MinContent,
    /// The `max-content` keyword.
    MaxContent,
    /// The bare `fit-content` keyword.
    FitContent,
    /// The `fit-content()` function with a non-negative argument.
    FitContentFunction(NonNegativeLengthPercentageValue),
    /// The `stretch` keyword.
    Stretch,
    /// The `contain` keyword.
    Contain,
}

impl From<NonNegativeLengthPercentage> for Size {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::LengthPercentage(value)
    }
}

impl Size {
    /// Construct `fit-content()` with a directly non-negative argument.
    pub fn fit_content(value: CssDimension) -> Self {
        Self::FitContentFunction(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct `fit-content()` with a typed calculation.
    pub fn fit_content_calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::FitContentFunction(LengthPercentageCalculation::new(expression).into())
    }
}

impl CssWriteTo for Size {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::LengthPercentage(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
            Self::MinContent => w.write_str("min-content"),
            Self::MaxContent => w.write_str("max-content"),
            Self::FitContent => w.write_str("fit-content"),
            Self::FitContentFunction(value) => {
                w.write_str("fit-content(")?;
                value.css_fmt(w)?;
                w.write_char(')')
            }
            Self::Stretch => w.write_str("stretch"),
            Self::Contain => w.write_str("contain"),
        }
    }
}

/// Values supported by the typed maximum-size property APIs.
///
/// The `stretch`, bare `fit-content`, and `contain` keywords extend the CSS Sizing Level 3
/// grammar as defined by CSS Sizing Level 4.
///
/// [CSS Sizing Level 3]: https://www.w3.org/TR/css-sizing-3/#sizing-values
/// [CSS Sizing Level 4]: https://www.w3.org/TR/css-sizing-4/#sizing-values
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MaxSize {
    /// The `none` keyword.
    None,
    /// A non-negative length or percentage.
    LengthPercentage(NonNegativeLengthPercentage),
    /// A typed length-percentage expression.
    Calculation(LengthPercentageCalculation),
    /// The `min-content` keyword.
    MinContent,
    /// The `max-content` keyword.
    MaxContent,
    /// The bare `fit-content` keyword.
    FitContent,
    /// The `fit-content()` function with a non-negative argument.
    FitContentFunction(NonNegativeLengthPercentageValue),
    /// The `stretch` keyword.
    Stretch,
    /// The `contain` keyword.
    Contain,
}

impl From<NonNegativeLengthPercentage> for MaxSize {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::LengthPercentage(value)
    }
}

impl MaxSize {
    /// Construct `fit-content()` with a directly non-negative argument.
    pub fn fit_content(value: CssDimension) -> Self {
        Self::FitContentFunction(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct `fit-content()` with a typed calculation.
    pub fn fit_content_calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::FitContentFunction(LengthPercentageCalculation::new(expression).into())
    }
}

impl CssWriteTo for MaxSize {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::None => w.write_str("none"),
            Self::LengthPercentage(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
            Self::MinContent => w.write_str("min-content"),
            Self::MaxContent => w.write_str("max-content"),
            Self::FitContent => w.write_str("fit-content"),
            Self::FitContentFunction(value) => {
                w.write_str("fit-content(")?;
                value.css_fmt(w)?;
                w.write_char(')')
            }
            Self::Stretch => w.write_str("stretch"),
            Self::Contain => w.write_str("contain"),
        }
    }
}

impl_checked_value!(Size, MaxSize);
