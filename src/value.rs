use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{
    CssAngle, CssColor, CssLength, CssTime, FiniteF64, GlobalKeyword, InvalidCssNumber,
    NonNegativeFiniteF64,
    number::{assert_finite, try_finite},
    serialization::CssWriteTo,
};

/// A closed union of typed CSS component values.
///
/// This type is useful for storing heterogeneous primitives. It is not a declaration grammar:
/// property-specific APIs accept narrower types and are the only values that integrate directly
/// with Leptos style attributes.
///
/// # Ergonomics
///
/// Use the free convenience functions for common values:
/// ```rust
/// use leptos_css::{px, em, rem, pct, deg};
///
/// let width = px(100);
/// let margin = em(0.6);
/// let offset = pct(50);
/// let rotation = deg(45);
/// ```
///
/// Arbitrary strings and unchecked function bodies are intentionally absent. Unsupported CSS
/// must be written outside this typed API until its grammar is modeled.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssValue {
    /// A bare floating-point number (e.g., opacity, flex-grow).
    Number(FiniteF64),
    /// A bare integer (e.g., z-index, order).
    Integer(i32),
    /// A CSS length value (px, em, rem, vh, vw, etc.).
    Length(CssLength),
    /// A percentage value.
    Percent(FiniteF64),
    /// A CSS angle value (deg, rad, turn, grad).
    Angle(CssAngle),
    /// A CSS time value (s, ms).
    Time(CssTime),
    /// A CSS color value (rgb, rgba, hsl, hsla, named).
    Color(CssColor),
    /// Fractional units for CSS Grid (e.g., `1fr`).
    Fr(NonNegativeFiniteF64),
    /// The CSS `auto` keyword.
    Auto,
    /// Zero with unit for `calc()` compatibility. Renders as `"0px"`.
    Zero,
    /// The CSS `inherit` keyword.
    Inherit,
    /// The CSS `initial` keyword.
    Initial,
    /// The CSS `unset` keyword.
    Unset,
    /// The CSS `revert` keyword.
    Revert,
    /// The CSS `revert-layer` keyword.
    RevertLayer,
}

impl CssWriteTo for CssValue {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Number(v) => write!(w, "{v}"),
            Self::Integer(v) => write!(w, "{v}"),
            Self::Length(l) => l.css_fmt(w),
            Self::Percent(v) => write!(w, "{v}%"),
            Self::Angle(a) => a.css_fmt(w),
            Self::Time(t) => t.css_fmt(w),
            Self::Color(c) => c.css_fmt(w),
            Self::Fr(v) => write!(w, "{v}fr"),
            Self::Auto => w.write_str("auto"),
            Self::Zero => w.write_str("0px"),
            Self::Inherit => w.write_str("inherit"),
            Self::Initial => w.write_str("initial"),
            Self::Unset => w.write_str("unset"),
            Self::Revert => w.write_str("revert"),
            Self::RevertLayer => w.write_str("revert-layer"),
        }
    }
}

// --- From impls for CssValue ---

impl From<CssLength> for CssValue {
    fn from(l: CssLength) -> Self {
        Self::Length(l)
    }
}

impl From<CssAngle> for CssValue {
    fn from(a: CssAngle) -> Self {
        Self::Angle(a)
    }
}

impl From<CssTime> for CssValue {
    fn from(t: CssTime) -> Self {
        Self::Time(t)
    }
}

impl From<CssColor> for CssValue {
    fn from(c: CssColor) -> Self {
        Self::Color(c)
    }
}

impl From<i32> for CssValue {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl From<FiniteF64> for CssValue {
    fn from(v: FiniteF64) -> Self {
        Self::Number(v)
    }
}

impl From<GlobalKeyword> for CssValue {
    fn from(keyword: GlobalKeyword) -> Self {
        match keyword {
            GlobalKeyword::Inherit => Self::Inherit,
            GlobalKeyword::Initial => Self::Initial,
            GlobalKeyword::Unset => Self::Unset,
            GlobalKeyword::Revert => Self::Revert,
            GlobalKeyword::RevertLayer => Self::RevertLayer,
        }
    }
}

impl TryFrom<f64> for CssValue {
    type Error = InvalidCssNumber;

    fn try_from(v: f64) -> Result<Self, Self::Error> {
        FiniteF64::try_new(v).map(Self::Number)
    }
}

impl From<CssValue> for String {
    fn from(value: CssValue) -> Self {
        let mut s = Self::new();
        value.write_to(&mut s);
        s
    }
}

impl From<CssValue> for std::borrow::Cow<'static, str> {
    fn from(value: CssValue) -> Self {
        Self::Owned(String::from(value))
    }
}

/// Create a bare CSS number.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite. Use [`try_number`] for runtime-derived values.
#[track_caller]
pub fn number(v: impl Into<f64>) -> CssValue {
    CssValue::Number(assert_finite(v.into(), "number"))
}

/// Attempt to create a finite bare CSS number.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_number(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "number").map(CssValue::Number)
}

/// Create a CSS degree angle value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn deg(v: impl Into<f64>) -> CssValue {
    CssValue::Angle(CssAngle::Deg(assert_finite(v.into(), "deg")))
}

/// Attempt to create a finite CSS degree angle.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_deg(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "deg").map(|v| CssValue::Angle(CssAngle::Deg(v)))
}

/// Create a CSS radian angle value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn rad(v: impl Into<f64>) -> CssValue {
    CssValue::Angle(CssAngle::Rad(assert_finite(v.into(), "rad")))
}

/// Attempt to create a finite CSS radian angle.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_rad(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "rad").map(|v| CssValue::Angle(CssAngle::Rad(v)))
}

/// Create a CSS turn angle value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn turn(v: impl Into<f64>) -> CssValue {
    CssValue::Angle(CssAngle::Turn(assert_finite(v.into(), "turn")))
}

/// Attempt to create a finite CSS turn angle.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_turn(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "turn").map(|v| CssValue::Angle(CssAngle::Turn(v)))
}

/// Create a CSS gradian angle value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn grad(v: impl Into<f64>) -> CssValue {
    CssValue::Angle(CssAngle::Grad(assert_finite(v.into(), "grad")))
}

/// Attempt to create a finite CSS gradian angle.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_grad(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "grad").map(|v| CssValue::Angle(CssAngle::Grad(v)))
}

/// Create a CSS seconds time value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn s(v: impl Into<f64>) -> CssValue {
    CssValue::Time(CssTime::S(assert_finite(v.into(), "s")))
}

/// Attempt to create a finite CSS seconds value.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_s(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "s").map(|v| CssValue::Time(CssTime::S(v)))
}

/// Create a CSS milliseconds time value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn ms(v: impl Into<f64>) -> CssValue {
    CssValue::Time(CssTime::Ms(assert_finite(v.into(), "ms")))
}

/// Attempt to create a finite CSS milliseconds value.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_ms(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    try_finite(v.into(), "ms").map(|v| CssValue::Time(CssTime::Ms(v)))
}

/// Create a CSS fractional unit value (for CSS Grid).
///
/// # Panics
///
/// Panics if `v` is `NaN`, infinite, or negative.
pub fn fr(v: impl Into<f64>) -> CssValue {
    CssValue::Fr(NonNegativeFiniteF64::new(v.into()))
}

/// Attempt to create a finite, non-negative CSS fractional unit value.
///
/// # Errors
///
/// Returns an error if `v` is `NaN`, infinite, or negative.
pub fn try_fr(v: impl Into<f64>) -> Result<CssValue, InvalidCssNumber> {
    NonNegativeFiniteF64::try_new(v.into()).map(CssValue::Fr)
}

impl_checked_value!(CssValue);
