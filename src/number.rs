use std::fmt;

/// Error returned when a floating-point value cannot be represented by a typed CSS value.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum InvalidCssNumber {
    /// The value is `NaN` or positive/negative infinity.
    NonFinite {
        /// The rejected value.
        value: f64,
        /// The CSS unit or channel being constructed.
        context: &'static str,
    },
    /// The value is finite but outside the permitted inclusive range.
    OutOfRange {
        /// The rejected value.
        value: f64,
        /// The CSS unit or channel being constructed.
        context: &'static str,
        /// The inclusive lower bound.
        min: f64,
        /// The inclusive upper bound.
        max: f64,
    },
    /// The value is finite but negative where CSS requires a non-negative number.
    Negative {
        /// The rejected value.
        value: f64,
        /// The CSS unit or channel being constructed.
        context: &'static str,
    },
}

impl fmt::Display for InvalidCssNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { value, context } => {
                write!(f, "CSS {context} value must be finite, got {value}")
            }
            Self::OutOfRange {
                value,
                context,
                min,
                max,
            } => write!(
                f,
                "CSS {context} must be in range [{min}, {max}], got {value}"
            ),
            Self::Negative { value, context } => {
                write!(f, "CSS {context} must be non-negative, got {value}")
            }
        }
    }
}

impl std::error::Error for InvalidCssNumber {}

/// A finite IEEE-754 double used by typed CSS numeric values.
///
/// The private field makes `NaN` and positive/negative infinity unrepresentable through the
/// public API. Use [`FiniteF64::try_new`] for runtime-derived values and [`FiniteF64::new`] for
/// values whose validity is a programming invariant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteF64(f64);

pub(crate) const fn try_finite(
    value: f64,
    context: &'static str,
) -> Result<FiniteF64, InvalidCssNumber> {
    if value.is_finite() {
        Ok(FiniteF64(value))
    } else {
        Err(InvalidCssNumber::NonFinite { value, context })
    }
}

impl FiniteF64 {
    /// Construct a finite value, panicking if `value` is `NaN` or infinite.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not finite.
    #[track_caller]
    pub const fn new(value: f64) -> Self {
        assert!(value.is_finite(), "CSS numeric value must be finite");
        Self(value)
    }

    /// Attempt to construct a finite value.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidCssNumber::NonFinite`] if `value` is `NaN` or infinite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_finite(value, "number")
    }

    /// Return the contained finite `f64`.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for FiniteF64 {
    type Error = InvalidCssNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<FiniteF64> for f64 {
    fn from(value: FiniteF64) -> Self {
        value.get()
    }
}

impl fmt::Display for FiniteF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Neg for FiniteF64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

/// A finite, non-negative number used by CSS grammars with a lower bound of zero.
///
/// The inner value is private, so negative and non-finite values cannot be represented.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonNegativeFiniteF64(FiniteF64);

impl NonNegativeFiniteF64 {
    /// Construct a non-negative finite number.
    ///
    /// # Panics
    ///
    /// Panics if `value` is negative, `NaN`, or infinite.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a non-negative finite number.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_non_negative(value, "non-negative number").map(Self)
    }

    /// Return the contained number.
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl fmt::Display for NonNegativeFiniteF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A finite number in the inclusive range `0..=1`.
///
/// This is the range used by alpha channels and normalized opacity values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitInterval(FiniteF64);

impl UnitInterval {
    /// Construct a value in the inclusive range `0..=1`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside the range or is not finite.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a value in the inclusive range `0..=1`.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_in_range(value, "unit interval", 0.0, 1.0).map(Self)
    }

    /// Return the contained number.
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl fmt::Display for UnitInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A finite percentage channel in the inclusive range `0..=100`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PercentageChannel(FiniteF64);

impl PercentageChannel {
    /// Construct a percentage channel in the inclusive range `0..=100`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside the range or is not finite.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a percentage channel in the inclusive range `0..=100`.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_in_range(value, "percentage channel", 0.0, 100.0).map(Self)
    }

    /// Return the contained percentage.
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl fmt::Display for PercentageChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[inline]
#[track_caller]
pub(crate) fn assert_finite(v: f64, context: &'static str) -> FiniteF64 {
    try_finite(v, context).unwrap_or_else(|error| panic!("{error}"))
}

#[inline]
fn try_in_range(
    v: f64,
    context: &'static str,
    min: f64,
    max: f64,
) -> Result<FiniteF64, InvalidCssNumber> {
    let finite = try_finite(v, context)?;
    if (min..=max).contains(&v) {
        Ok(finite)
    } else {
        Err(InvalidCssNumber::OutOfRange {
            value: v,
            context,
            min,
            max,
        })
    }
}

#[inline]
fn try_non_negative(v: f64, context: &'static str) -> Result<FiniteF64, InvalidCssNumber> {
    let finite = try_finite(v, context)?;
    if v >= 0.0 {
        Ok(finite)
    } else {
        Err(InvalidCssNumber::Negative { value: v, context })
    }
}
