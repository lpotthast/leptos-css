use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{
    CssLength, CssValue, CssVariableReference, FiniteF64, InvalidCssNumber, number::try_finite,
    serialization::CssWriteTo,
};

/// A CSS `<length-percentage>` value.
///
/// This is a type-safe subset of [`CssValue`] for component props that specifically need
/// a length or percentage. Property grammars that also accept `auto` use
/// [`LengthPercentageAuto`] instead, so `auto` cannot leak into arithmetic or padding.
///
/// Converts to [`CssValue`] via `From`/`Into` for use with the style system.
///
/// # Example
/// ```rust
/// use leptos_css::{CssDimension, px, pct};
///
/// let gap = px(10);
/// let width = pct(100);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssDimension {
    /// A CSS length value (px, em, rem, vh, vw, etc.).
    Length(CssLength),
    /// A percentage value.
    Percent(FiniteF64),
    /// Zero with unit for `calc()` compatibility. Renders as `"0px"`.
    Zero,
}

/// Const constructors for [`CssDimension`].
///
/// Each constructor wraps a [`CssLength`] variant or [`CssDimension::Percent`]; see the
/// crate-level helpers in [`crate`] (`px`, `em`, `rem`, …) for the equivalent free functions.
macro_rules! dimension_constructor {
    ($(#[$meta:meta])* $name:ident, $try_name:ident, $variant:ident, $unit:literal) => {
        $(#[$meta])*
        ///
        /// # Panics
        ///
        /// Panics if `v` is `NaN` or infinite. Use the corresponding `try_` constructor for
        /// runtime-derived values.
        #[track_caller]
        pub const fn $name(v: f64) -> Self {
            match Self::$try_name(v) {
                Ok(value) => value,
                Err(_) => panic!(concat!("CSS ", $unit, " value must be finite")),
            }
        }

        #[doc = concat!("Attempt to construct a finite `", $unit, "` dimension.")]
        ///
        /// # Errors
        ///
        /// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
        pub const fn $try_name(v: f64) -> Result<Self, InvalidCssNumber> {
            match try_finite(v, $unit) {
                Ok(value) => Ok(Self::Length(CssLength::$variant(value))),
                Err(error) => Err(error),
            }
        }
    };
}

impl CssDimension {
    dimension_constructor!(/// Construct a `px` length dimension.
        px, try_px, Px, "px");
    dimension_constructor!(/// Construct an `em` length dimension.
        em, try_em, Em, "em");
    dimension_constructor!(/// Construct a `rem` length dimension.
        rem, try_rem, Rem, "rem");
    dimension_constructor!(/// Construct a `vw` length dimension.
        vw, try_vw, Vw, "vw");
    dimension_constructor!(/// Construct a `vh` length dimension.
        vh, try_vh, Vh, "vh");
    dimension_constructor!(/// Construct a `vmin` length dimension.
        vmin, try_vmin, Vmin, "vmin");
    dimension_constructor!(/// Construct a `vmax` length dimension.
        vmax, try_vmax, Vmax, "vmax");
    dimension_constructor!(/// Construct a `ch` length dimension.
        ch, try_ch, Ch, "ch");
    dimension_constructor!(/// Construct a `dvw` length dimension.
        dvw, try_dvw, Dvw, "dvw");
    dimension_constructor!(/// Construct a `dvh` length dimension.
        dvh, try_dvh, Dvh, "dvh");
    dimension_constructor!(/// Construct an `svw` length dimension.
        svw, try_svw, Svw, "svw");
    dimension_constructor!(/// Construct an `svh` length dimension.
        svh, try_svh, Svh, "svh");
    dimension_constructor!(/// Construct an `lvw` length dimension.
        lvw, try_lvw, Lvw, "lvw");
    dimension_constructor!(/// Construct an `lvh` length dimension.
        lvh, try_lvh, Lvh, "lvh");
    dimension_constructor!(/// Construct a `cqw` length dimension.
        cqw, try_cqw, Cqw, "cqw");
    dimension_constructor!(/// Construct a `cqh` length dimension.
        cqh, try_cqh, Cqh, "cqh");

    /// Construct a percentage dimension, panicking if `v` is not finite.
    ///
    /// # Panics
    ///
    /// Panics if `v` is `NaN` or infinite. Use [`CssDimension::try_pct`] for runtime-derived
    /// values.
    #[track_caller]
    pub const fn pct(v: f64) -> Self {
        match Self::try_pct(v) {
            Ok(value) => value,
            Err(_) => panic!("CSS % value must be finite"),
        }
    }

    /// Attempt to construct a finite percentage dimension.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
    pub const fn try_pct(v: f64) -> Result<Self, InvalidCssNumber> {
        match try_finite(v, "%") {
            Ok(value) => Ok(Self::Percent(value)),
            Err(error) => Err(error),
        }
    }
}

impl CssWriteTo for CssDimension {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Length(l) => l.css_fmt(w),
            Self::Percent(v) => write!(w, "{v}%"),
            Self::Zero => w.write_str("0px"),
        }
    }
}

impl From<CssDimension> for CssValue {
    fn from(dim: CssDimension) -> Self {
        match dim {
            CssDimension::Length(l) => Self::Length(l),
            CssDimension::Percent(v) => Self::Percent(v),
            CssDimension::Zero => Self::Zero,
        }
    }
}

impl From<CssLength> for CssDimension {
    fn from(l: CssLength) -> Self {
        Self::Length(l)
    }
}

/// A `<length-percentage> | auto` value for grammars such as margin and inset offsets.
///
/// Typed calculations are also accepted because CSS math functions resolve to a
/// `<length-percentage>` in these contexts.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum LengthPercentageAuto {
    /// A length or percentage.
    LengthPercentage(CssDimension),
    /// A typed length-percentage calculation.
    Calculation(CssDimensionExpr),
    /// The `auto` keyword.
    Auto,
}

impl From<CssDimension> for LengthPercentageAuto {
    fn from(value: CssDimension) -> Self {
        Self::LengthPercentage(value)
    }
}

impl From<CssDimensionExpr> for LengthPercentageAuto {
    fn from(value: CssDimensionExpr) -> Self {
        Self::Calculation(value)
    }
}

impl CssWriteTo for LengthPercentageAuto {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
            Self::Auto => w.write_str("auto"),
        }
    }
}

/// User-agent environment variables supported by the typed `env()` expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CssEnvironmentVariable {
    /// `safe-area-inset-top`.
    SafeAreaInsetTop,
    /// `safe-area-inset-right`.
    SafeAreaInsetRight,
    /// `safe-area-inset-bottom`.
    SafeAreaInsetBottom,
    /// `safe-area-inset-left`.
    SafeAreaInsetLeft,
    /// `safe-area-max-inset-top`.
    SafeAreaMaxInsetTop,
    /// `safe-area-max-inset-right`.
    SafeAreaMaxInsetRight,
    /// `safe-area-max-inset-bottom`.
    SafeAreaMaxInsetBottom,
    /// `safe-area-max-inset-left`.
    SafeAreaMaxInsetLeft,
}

impl CssEnvironmentVariable {
    const fn as_str(self) -> &'static str {
        match self {
            Self::SafeAreaInsetTop => "safe-area-inset-top",
            Self::SafeAreaInsetRight => "safe-area-inset-right",
            Self::SafeAreaInsetBottom => "safe-area-inset-bottom",
            Self::SafeAreaInsetLeft => "safe-area-inset-left",
            Self::SafeAreaMaxInsetTop => "safe-area-max-inset-top",
            Self::SafeAreaMaxInsetRight => "safe-area-max-inset-right",
            Self::SafeAreaMaxInsetBottom => "safe-area-max-inset-bottom",
            Self::SafeAreaMaxInsetLeft => "safe-area-max-inset-left",
        }
    }
}

/// A typed CSS length-percentage expression.
///
/// The expression tree is private. Values can only be constructed from typed dimensions,
/// arithmetic, or the typed [`css_min`], [`css_max`], [`css_clamp`], and [`css_env`] helpers.
/// This prevents malformed function bodies and invalid argument counts from being represented.
#[derive(Clone, Debug, PartialEq)]
pub struct CssDimensionExpr(DimensionExpr);

#[derive(Clone, Debug, PartialEq)]
enum DimensionExpr {
    Simple(CssDimension),
    Variable(CssVariableReference<CssDimension>),
    Add(Box<CssDimensionExpr>, Box<CssDimensionExpr>),
    Subtract(Box<CssDimensionExpr>, Box<CssDimensionExpr>),
    Min(Box<CssDimensionExpr>, Box<CssDimensionExpr>),
    Max(Box<CssDimensionExpr>, Box<CssDimensionExpr>),
    Clamp {
        min: Box<CssDimensionExpr>,
        preferred: Box<CssDimensionExpr>,
        max: Box<CssDimensionExpr>,
    },
    Env(CssEnvironmentVariable),
}

impl CssDimensionExpr {
    fn calc_fmt<W: fmt::Write>(&self, w: &mut W, nested: bool) -> fmt::Result {
        match &self.0 {
            DimensionExpr::Simple(value) => value.css_fmt(w),
            DimensionExpr::Variable(value) => value.css_fmt(w),
            DimensionExpr::Add(lhs, rhs) | DimensionExpr::Subtract(lhs, rhs) => {
                if nested {
                    w.write_char('(')?;
                }
                lhs.calc_fmt(w, true)?;
                if matches!(&self.0, DimensionExpr::Add(_, _)) {
                    w.write_str(" + ")?;
                } else {
                    w.write_str(" - ")?;
                }
                rhs.calc_fmt(w, true)?;
                if nested {
                    w.write_char(')')?;
                }
                Ok(())
            }
            DimensionExpr::Min(_, _)
            | DimensionExpr::Max(_, _)
            | DimensionExpr::Clamp { .. }
            | DimensionExpr::Env(_) => self.css_fmt(w),
        }
    }

    /// Serialize as a top-level calculation for a range-restricted property grammar.
    ///
    /// Binary nodes already emit `calc()`, while `min()`, `max()`, and `clamp()` emit their own CSS
    /// math functions. Simple dimensions, `var()`, and `env()` need an explicit `calc()` wrapper
    /// so substituted or folded out-of-range results are clamped under CSS math-function range
    /// checking instead of becoming invalid plain values.
    fn css_fmt_as_top_level_calculation<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if matches!(
            &self.0,
            DimensionExpr::Simple(_) | DimensionExpr::Variable(_) | DimensionExpr::Env(_)
        ) {
            w.write_str("calc(")?;
            self.calc_fmt(w, false)?;
            w.write_char(')')
        } else {
            self.css_fmt(w)
        }
    }
}

impl CssWriteTo for CssDimensionExpr {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match &self.0 {
            DimensionExpr::Simple(value) => value.css_fmt(w),
            DimensionExpr::Variable(value) => value.css_fmt(w),
            DimensionExpr::Add(_, _) | DimensionExpr::Subtract(_, _) => {
                w.write_str("calc(")?;
                self.calc_fmt(w, false)?;
                w.write_char(')')
            }
            DimensionExpr::Min(lhs, rhs) => {
                w.write_str("min(")?;
                lhs.css_fmt(w)?;
                w.write_str(", ")?;
                rhs.css_fmt(w)?;
                w.write_char(')')
            }
            DimensionExpr::Max(lhs, rhs) => {
                w.write_str("max(")?;
                lhs.css_fmt(w)?;
                w.write_str(", ")?;
                rhs.css_fmt(w)?;
                w.write_char(')')
            }
            DimensionExpr::Clamp {
                min,
                preferred,
                max,
            } => {
                w.write_str("clamp(")?;
                min.css_fmt(w)?;
                w.write_str(", ")?;
                preferred.css_fmt(w)?;
                w.write_str(", ")?;
                max.css_fmt(w)?;
                w.write_char(')')
            }
            DimensionExpr::Env(variable) => write!(w, "env({})", variable.as_str()),
        }
    }
}

impl From<CssDimension> for CssDimensionExpr {
    fn from(dim: CssDimension) -> Self {
        Self(DimensionExpr::Simple(dim))
    }
}

impl From<CssLength> for CssDimensionExpr {
    fn from(length: CssLength) -> Self {
        Self::from(CssDimension::Length(length))
    }
}

impl From<CssVariableReference<CssDimension>> for CssDimensionExpr {
    fn from(variable: CssVariableReference<CssDimension>) -> Self {
        Self(DimensionExpr::Variable(variable))
    }
}

/// A length-percentage math expression suitable for a property grammar.
///
/// This wrapper guarantees that a simple operand is emitted inside `calc()`. That distinction
/// matters for range-restricted grammars: `calc(-1px)` is valid and is clamped by
/// [CSS math-function range checking], while a plain `-1px` can be invalid for the same property.
///
/// [CSS math-function range checking]: https://drafts.csswg.org/css-values-4/#calc-range
#[derive(Clone, Debug, PartialEq)]
pub struct LengthPercentageCalculation(CssDimensionExpr);

impl LengthPercentageCalculation {
    /// Construct a checked length-percentage calculation.
    ///
    pub fn new(expression: impl Into<CssDimensionExpr>) -> Self {
        Self(expression.into())
    }
}

impl CssWriteTo for LengthPercentageCalculation {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.0.css_fmt_as_top_level_calculation(w)
    }
}

impl From<CssDimension> for String {
    fn from(dim: CssDimension) -> Self {
        let mut s = Self::new();
        dim.write_to(&mut s);
        s
    }
}

impl From<CssDimension> for std::borrow::Cow<'static, str> {
    fn from(dim: CssDimension) -> Self {
        Self::Owned(String::from(dim))
    }
}

impl From<CssDimensionExpr> for String {
    fn from(expr: CssDimensionExpr) -> Self {
        let mut s = Self::new();
        expr.write_to(&mut s);
        s
    }
}

impl From<CssDimensionExpr> for std::borrow::Cow<'static, str> {
    fn from(expr: CssDimensionExpr) -> Self {
        Self::Owned(String::from(expr))
    }
}

impl std::ops::Neg for CssDimension {
    type Output = Self;

    /// Negate a dimension value.
    fn neg(self) -> Self {
        match self {
            Self::Length(l) => Self::Length(-l),
            Self::Percent(v) => Self::Percent(-v),
            Self::Zero => Self::Zero,
        }
    }
}

// --- Dimension arithmetic ---

/// Returns true if two CssLength values use the same unit (ignoring the numeric value).
fn same_unit(a: &CssLength, b: &CssLength) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

impl std::ops::Add for CssDimension {
    type Output = CssDimensionExpr;

    /// Add two dimension values. Same-unit operations fold (e.g. `px(10) + px(5)` = `px(15)`).
    /// Mixed-unit operations produce a `calc()` expression.
    ///
    /// # Panics
    ///
    /// Panics if folding same-unit operands overflows to a non-finite result.
    fn add(self, rhs: Self) -> CssDimensionExpr {
        match (self, rhs) {
            (Self::Zero, other) | (other, Self::Zero) => CssDimensionExpr::from(other),
            (Self::Length(a), Self::Length(b)) if same_unit(&a, &b) => {
                CssDimensionExpr::from(Self::Length(a.map_value(|v| v + b.value())))
            }
            (Self::Percent(a), Self::Percent(b)) => {
                CssDimensionExpr::from(Self::Percent(FiniteF64::new(a.get() + b.get())))
            }
            (a, b) => CssDimensionExpr(DimensionExpr::Add(Box::new(a.into()), Box::new(b.into()))),
        }
    }
}

impl std::ops::Sub for CssDimension {
    type Output = CssDimensionExpr;

    /// Subtract two dimension values. Same-unit operations fold (e.g. `px(10) - px(5)` = `px(5)`).
    /// Mixed-unit operations produce a `calc()` expression.
    ///
    /// # Panics
    ///
    /// Panics if folding same-unit operands overflows to a non-finite result.
    fn sub(self, rhs: Self) -> CssDimensionExpr {
        match (self, rhs) {
            (lhs, Self::Zero) => CssDimensionExpr::from(lhs),
            (Self::Zero, rhs) => CssDimensionExpr::from(-rhs),
            (Self::Length(a), Self::Length(b)) if same_unit(&a, &b) => {
                CssDimensionExpr::from(Self::Length(a.map_value(|v| v - b.value())))
            }
            (Self::Percent(a), Self::Percent(b)) => {
                CssDimensionExpr::from(Self::Percent(FiniteF64::new(a.get() - b.get())))
            }
            (a, b) => CssDimensionExpr(DimensionExpr::Subtract(
                Box::new(a.into()),
                Box::new(b.into()),
            )),
        }
    }
}

impl std::ops::Add<CssDimension> for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn add(self, rhs: CssDimension) -> CssDimensionExpr {
        self + CssDimensionExpr::from(rhs)
    }
}

impl std::ops::Sub<CssDimension> for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn sub(self, rhs: CssDimension) -> CssDimensionExpr {
        self - CssDimensionExpr::from(rhs)
    }
}

impl std::ops::Add for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn add(self, rhs: Self) -> Self {
        match (self.0, rhs.0) {
            (DimensionExpr::Simple(lhs), DimensionExpr::Simple(rhs)) => lhs + rhs,
            (lhs, rhs) => Self(DimensionExpr::Add(Box::new(Self(lhs)), Box::new(Self(rhs)))),
        }
    }
}

impl std::ops::Sub for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn sub(self, rhs: Self) -> Self {
        match (self.0, rhs.0) {
            (DimensionExpr::Simple(lhs), DimensionExpr::Simple(rhs)) => lhs - rhs,
            (lhs, rhs) => Self(DimensionExpr::Subtract(
                Box::new(Self(lhs)),
                Box::new(Self(rhs)),
            )),
        }
    }
}

macro_rules! dim_fn {
    ($(#[$meta:meta])* $fn_name:ident, $try_fn_name:ident, $unit_str:literal) => {
        $(#[$meta])*
        ///
        /// # Panics
        ///
        /// Panics if `v` is `NaN` or infinite. Use the corresponding `try_` helper for
        /// runtime-derived values.
        #[track_caller]
        pub fn $fn_name(v: impl Into<f64>) -> CssDimension {
            CssDimension::$fn_name(v.into())
        }

        #[doc = concat!("Attempt to create a finite CSS `", $unit_str, "` value.")]
        ///
        /// # Errors
        ///
        /// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
        pub fn $try_fn_name(v: impl Into<f64>) -> Result<CssDimension, InvalidCssNumber> {
            CssDimension::$try_fn_name(v.into())
        }
    };
}

dim_fn!(/// Create a CSS pixel value.
    px, try_px, "px");
dim_fn!(/// Create a CSS em value (relative to the element's font size).
    em, try_em, "em");
dim_fn!(
    #[allow(clippy::module_name_repetitions)]
    /// Create a CSS rem value (relative to the root element's font size).
    rem,
    try_rem,
    "rem"
);
dim_fn!(/// Create a CSS viewport-width value (1vw = 1% of the viewport width).
    vw, try_vw, "vw");
dim_fn!(/// Create a CSS viewport-height value (1vh = 1% of the viewport height).
    vh, try_vh, "vh");
dim_fn!(/// Create a CSS vmin value (1vmin = 1% of the smaller viewport dimension).
    vmin, try_vmin, "vmin");
dim_fn!(/// Create a CSS vmax value (1vmax = 1% of the larger viewport dimension).
    vmax, try_vmax, "vmax");
dim_fn!(/// Create a CSS ch (character width) value — the width of the `0` glyph in the element's font.
    ch, try_ch, "ch");
dim_fn!(/// Create a CSS dynamic viewport width value.
    /// Unlike `vw`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    dvw, try_dvw, "dvw");
dim_fn!(/// Create a CSS dynamic viewport height value.
    /// Unlike `vh`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    dvh, try_dvh, "dvh");
dim_fn!(/// Create a CSS small viewport width value.
    /// Represents the viewport width when all dynamic browser UI is expanded/visible.
    svw, try_svw, "svw");
dim_fn!(/// Create a CSS small viewport height value.
    /// Represents the viewport height when all dynamic browser UI is expanded/visible.
    svh, try_svh, "svh");
dim_fn!(/// Create a CSS large viewport width value.
    /// Represents the viewport width when all dynamic browser UI is retracted/hidden.
    lvw, try_lvw, "lvw");
dim_fn!(/// Create a CSS large viewport height value.
    /// Represents the viewport height when all dynamic browser UI is retracted/hidden.
    lvh, try_lvh, "lvh");
dim_fn!(/// Create a CSS container query width value (1cqw = 1% of the nearest size container's width).
    cqw, try_cqw, "cqw");
dim_fn!(/// Create a CSS container query height value (1cqh = 1% of the nearest size container's height).
    cqh, try_cqh, "cqh");

/// Create a CSS percentage value.
///
/// # Panics
///
/// Panics if `v` is `NaN` or infinite.
pub fn pct(v: impl Into<f64>) -> CssDimension {
    CssDimension::pct(v.into())
}

/// Attempt to create a finite CSS percentage value.
///
/// # Errors
///
/// Returns [`InvalidCssNumber::NonFinite`] if `v` is `NaN` or infinite.
pub fn try_pct(v: impl Into<f64>) -> Result<CssDimension, InvalidCssNumber> {
    CssDimension::try_pct(v.into())
}

/// Create a typed CSS `min()` length-percentage expression.
///
/// Both arguments are typed, so malformed bodies and empty argument lists are impossible.
pub fn css_min(
    lhs: impl Into<CssDimensionExpr>,
    rhs: impl Into<CssDimensionExpr>,
) -> CssDimensionExpr {
    let lhs = lhs.into();
    let rhs = rhs.into();
    CssDimensionExpr(DimensionExpr::Min(Box::new(lhs), Box::new(rhs)))
}

/// Create a typed CSS `max()` length-percentage expression.
pub fn css_max(
    lhs: impl Into<CssDimensionExpr>,
    rhs: impl Into<CssDimensionExpr>,
) -> CssDimensionExpr {
    let lhs = lhs.into();
    let rhs = rhs.into();
    CssDimensionExpr(DimensionExpr::Max(Box::new(lhs), Box::new(rhs)))
}

/// Create a typed CSS `clamp(min, preferred, max)` length-percentage expression.
pub fn css_clamp(
    min: impl Into<CssDimensionExpr>,
    preferred: impl Into<CssDimensionExpr>,
    max: impl Into<CssDimensionExpr>,
) -> CssDimensionExpr {
    let min = min.into();
    let preferred = preferred.into();
    let max = max.into();
    CssDimensionExpr(DimensionExpr::Clamp {
        min: Box::new(min),
        preferred: Box::new(preferred),
        max: Box::new(max),
    })
}

/// Create a typed CSS `env()` expression for a supported user-agent variable.
pub fn css_env(variable: CssEnvironmentVariable) -> CssDimensionExpr {
    CssDimensionExpr(DimensionExpr::Env(variable))
}

impl_checked_value!(
    CssDimension,
    LengthPercentageAuto,
    CssDimensionExpr,
    LengthPercentageCalculation,
);
