use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{
    CssDimension, CssDimensionExpr, DeclarationValue, LengthPercentageAuto,
    LengthPercentageCalculation, serialization::CssWriteTo,
};

macro_rules! box_model_shorthand {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            /// Only on the top edge (other edges are `0`).
            Top($value),
            /// Only on the right edge (other edges are `0`).
            Right($value),
            /// Only on the bottom edge (other edges are `0`).
            Bottom($value),
            /// Only on the left edge (other edges are `0`).
            Left($value),
            /// Equal on all four edges.
            All($value),
            /// Separate vertical (top/bottom) and horizontal (left/right).
            Double($value, $value),
            /// Explicit for each edge: top, right, bottom, left.
            Full($value, $value, $value, $value),
        }

        impl CssWriteTo for $name {
            fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    Self::Top(size) => {
                        size.css_fmt(w)?;
                        w.write_str(" 0 0 0")
                    }
                    Self::Right(size) => {
                        w.write_str("0 ")?;
                        size.css_fmt(w)?;
                        w.write_str(" 0 0")
                    }
                    Self::Bottom(size) => {
                        w.write_str("0 0 ")?;
                        size.css_fmt(w)?;
                        w.write_str(" 0")
                    }
                    Self::Left(size) => {
                        w.write_str("0 0 0 ")?;
                        size.css_fmt(w)
                    }
                    Self::All(size) => size.css_fmt(w),
                    Self::Double(vertical, horizontal) => {
                        vertical.css_fmt(w)?;
                        w.write_char(' ')?;
                        horizontal.css_fmt(w)
                    }
                    Self::Full(top, right, bottom, left) => {
                        top.css_fmt(w)?;
                        w.write_char(' ')?;
                        right.css_fmt(w)?;
                        w.write_char(' ')?;
                        bottom.css_fmt(w)?;
                        w.write_char(' ')?;
                        left.css_fmt(w)
                    }
                }
            }
        }

    };
}

box_model_shorthand!(
    /// CSS margin shorthand values.
    ///
    /// Represents the `margin` CSS shorthand as a typed enum. Each variant maps to
    /// a different shorthand form:
    ///
    /// - `All(dim)` → `margin: dim`
    /// - `Double(v, h)` → `margin: v h`
    /// - `Full(t, r, b, l)` → `margin: t r b l`
    /// - `Top(dim)` → `margin: dim 0 0 0` (and similarly for other sides)
    ///
    /// [CSS Box Model margin grammar]: https://www.w3.org/TR/css-box-3/#margin-physical
    Margin,
    LengthPercentageAuto
);

box_model_shorthand!(
    /// CSS `inset` shorthand values.
    ///
    /// Each component accepts `auto`, a direct length-percentage, or a typed calculation.
    ///
    /// [CSS Positioned Layout inset grammar]: https://www.w3.org/TR/css-position-3/#insets
    Inset,
    LengthPercentageAuto
);

/// A non-negative `<length-percentage>` suitable for grammars such as `padding` and `gap`.
///
/// `auto`, negative values, and non-finite values cannot be represented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonNegativeLengthPercentage(CssDimension);

impl NonNegativeLengthPercentage {
    /// Construct a non-negative length-percentage.
    ///
    /// # Panics
    ///
    /// Panics if `value` is negative.
    #[track_caller]
    pub fn new(value: CssDimension) -> Self {
        Self::try_from(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Return the underlying typed dimension.
    pub const fn get(self) -> CssDimension {
        self.0
    }
}

/// Error returned when a dimension is not a non-negative `<length-percentage>`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum InvalidNonNegativeLengthPercentage {
    /// The numeric value is negative.
    Negative(f64),
}

impl fmt::Display for InvalidNonNegativeLengthPercentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative(value) => write!(f, "CSS value must be non-negative, got {value}"),
        }
    }
}

impl std::error::Error for InvalidNonNegativeLengthPercentage {}

impl TryFrom<CssDimension> for NonNegativeLengthPercentage {
    type Error = InvalidNonNegativeLengthPercentage;

    fn try_from(value: CssDimension) -> Result<Self, Self::Error> {
        let number = match value {
            CssDimension::Length(length) => length.value(),
            CssDimension::Percent(number) => number.get(),
            CssDimension::Zero => 0.0,
        };
        if number < 0.0 {
            Err(InvalidNonNegativeLengthPercentage::Negative(number))
        } else {
            Ok(Self(value))
        }
    }
}

impl CssWriteTo for NonNegativeLengthPercentage {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.0.css_fmt(w)
    }
}

/// A direct or calculated `<length-percentage [0,∞]>` value.
///
/// Direct values are checked before construction. Calculations stay inside a CSS math function so
/// the specification's top-level range clamping applies after evaluation and substitution.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum NonNegativeLengthPercentageValue {
    /// A directly non-negative length or percentage.
    Direct(NonNegativeLengthPercentage),
    /// A typed calculation whose result is clamped to the property's non-negative range.
    Calculation(LengthPercentageCalculation),
}

impl From<NonNegativeLengthPercentage> for NonNegativeLengthPercentageValue {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::Direct(value)
    }
}

impl From<LengthPercentageCalculation> for NonNegativeLengthPercentageValue {
    fn from(value: LengthPercentageCalculation) -> Self {
        Self::Calculation(value)
    }
}

impl From<NonNegativeLengthPercentage> for DeclarationValue<NonNegativeLengthPercentageValue> {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        NonNegativeLengthPercentageValue::from(value).into()
    }
}

impl From<LengthPercentageCalculation> for DeclarationValue<NonNegativeLengthPercentageValue> {
    fn from(value: LengthPercentageCalculation) -> Self {
        NonNegativeLengthPercentageValue::from(value).into()
    }
}

impl CssWriteTo for NonNegativeLengthPercentageValue {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Direct(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
        }
    }
}

/// A logical or physical corner-radius longhand value.
///
/// The grammar is `<length-percentage [0,∞]>{1,2}`: one value creates a circular radius and two
/// values specify the horizontal and vertical radii of an ellipse.
///
/// [CSS Logical Properties § 4.6]: https://www.w3.org/TR/css-logical-1/#border-radius-properties
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum BorderCornerRadius {
    /// One radius used in both dimensions.
    Circular(NonNegativeLengthPercentageValue),
    /// Separate horizontal and vertical radii.
    Elliptical(
        NonNegativeLengthPercentageValue,
        NonNegativeLengthPercentageValue,
    ),
}

impl BorderCornerRadius {
    /// Construct a circular radius from a direct non-negative value.
    pub fn circular(value: CssDimension) -> Self {
        Self::Circular(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct an elliptical radius from direct non-negative values.
    pub fn elliptical(horizontal: CssDimension, vertical: CssDimension) -> Self {
        Self::Elliptical(
            NonNegativeLengthPercentage::new(horizontal).into(),
            NonNegativeLengthPercentage::new(vertical).into(),
        )
    }

    /// Construct a circular radius from a typed calculation.
    pub fn calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::Circular(LengthPercentageCalculation::new(expression).into())
    }
}

impl CssWriteTo for BorderCornerRadius {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Circular(radius) => radius.css_fmt(w),
            Self::Elliptical(horizontal, vertical) => {
                horizontal.css_fmt(w)?;
                w.write_char(' ')?;
                vertical.css_fmt(w)
            }
        }
    }
}

box_model_shorthand!(
    /// CSS padding shorthand values.
    ///
    /// Mirrors the [`Margin`] type but for the `padding` property.
    ///
    /// - `All(dim)` → `padding: dim`
    /// - `Double(v, h)` → `padding: v h`
    /// - `Full(t, r, b, l)` → `padding: t r b l`
    /// - `Top(dim)` → `padding: dim 0 0 0` (and similarly for other sides)
    ///
    /// [CSS Box Model padding grammar]: https://www.w3.org/TR/css-box-3/#padding-physical
    Padding,
    NonNegativeLengthPercentageValue
);

impl Padding {
    /// Construct equal padding on all four edges.
    #[track_caller]
    pub fn all(value: CssDimension) -> Self {
        Self::All(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct vertical and horizontal padding.
    #[track_caller]
    pub fn double(vertical: CssDimension, horizontal: CssDimension) -> Self {
        Self::Double(
            NonNegativeLengthPercentage::new(vertical).into(),
            NonNegativeLengthPercentage::new(horizontal).into(),
        )
    }

    /// Construct explicit top, right, bottom, and left padding.
    #[track_caller]
    pub fn full(
        top: CssDimension,
        right: CssDimension,
        bottom: CssDimension,
        left: CssDimension,
    ) -> Self {
        Self::Full(
            NonNegativeLengthPercentage::new(top).into(),
            NonNegativeLengthPercentage::new(right).into(),
            NonNegativeLengthPercentage::new(bottom).into(),
            NonNegativeLengthPercentage::new(left).into(),
        )
    }

    /// Construct equal calculated padding on all four edges.
    pub fn calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::All(LengthPercentageCalculation::new(expression).into())
    }
}

macro_rules! axis_shorthand {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            /// One value for both the start and end edges.
            One($value),
            /// Separate start-edge and end-edge values.
            StartEnd($value, $value),
        }

        impl CssWriteTo for $name {
            fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    Self::One(value) => value.css_fmt(w),
                    Self::StartEnd(start, end) => {
                        start.css_fmt(w)?;
                        w.write_char(' ')?;
                        end.css_fmt(w)
                    }
                }
            }
        }
    };
}

axis_shorthand!(
    /// Values for the `padding-block` and `padding-inline` shorthands.
    PaddingAxis,
    NonNegativeLengthPercentageValue
);

impl PaddingAxis {
    /// Construct equal direct padding on both axis edges.
    pub fn one(value: CssDimension) -> Self {
        Self::One(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct separate direct padding on the start and end edges.
    pub fn start_end(start: CssDimension, end: CssDimension) -> Self {
        Self::StartEnd(
            NonNegativeLengthPercentage::new(start).into(),
            NonNegativeLengthPercentage::new(end).into(),
        )
    }

    /// Construct equal calculated padding on both axis edges.
    pub fn calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::One(LengthPercentageCalculation::new(expression).into())
    }
}

axis_shorthand!(
    /// Values for the `margin-block` and `margin-inline` shorthands.
    MarginAxis,
    LengthPercentageAuto
);

axis_shorthand!(
    /// Values for the `inset-block` and `inset-inline` shorthands.
    InsetAxis,
    LengthPercentageAuto
);

impl_checked_value!(
    Margin,
    Inset,
    Padding,
    PaddingAxis,
    MarginAxis,
    InsetAxis,
    NonNegativeLengthPercentage,
    NonNegativeLengthPercentageValue,
    BorderCornerRadius,
);
