use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{FiniteF64, InvalidCssNumber, serialization::CssWriteTo};

/// CSS length values with absolute and relative units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssLength {
    /// Pixels.
    Px(FiniteF64),
    /// Em units (relative to element font size).
    Em(FiniteF64),
    /// Rem units (relative to root font size).
    Rem(FiniteF64),
    /// Viewport width units.
    Vw(FiniteF64),
    /// Viewport height units.
    Vh(FiniteF64),
    /// Viewport minimum dimension units.
    Vmin(FiniteF64),
    /// Viewport maximum dimension units.
    Vmax(FiniteF64),
    /// Character width units (width of the `0` glyph in the element's font).
    Ch(FiniteF64),
    /// Dynamic viewport width units (1dvw = 1% of the dynamic viewport width).
    /// Unlike `vw`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    Dvw(FiniteF64),
    /// Dynamic viewport height units (1dvh = 1% of the dynamic viewport height).
    /// Unlike `vh`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    Dvh(FiniteF64),
    /// Small viewport width units (1svw = 1% of the smallest possible viewport width).
    /// Represents the viewport size when all dynamic browser UI is expanded/visible.
    Svw(FiniteF64),
    /// Small viewport height units (1svh = 1% of the smallest possible viewport height).
    /// Represents the viewport size when all dynamic browser UI is expanded/visible.
    Svh(FiniteF64),
    /// Large viewport width units (1lvw = 1% of the largest possible viewport width).
    /// Represents the viewport size when all dynamic browser UI is retracted/hidden.
    Lvw(FiniteF64),
    /// Large viewport height units (1lvh = 1% of the largest possible viewport height).
    /// Represents the viewport size when all dynamic browser UI is retracted/hidden.
    Lvh(FiniteF64),
    /// Container query width units (1cqw = 1% of the nearest size container's width).
    /// Used with CSS container queries (`@container`).
    Cqw(FiniteF64),
    /// Container query height units (1cqh = 1% of the nearest size container's height).
    /// Used with CSS container queries (`@container`).
    Cqh(FiniteF64),
}

impl CssLength {
    /// Returns the inner numeric value regardless of unit.
    pub fn value(self) -> f64 {
        match self {
            Self::Px(v)
            | Self::Em(v)
            | Self::Rem(v)
            | Self::Vw(v)
            | Self::Vh(v)
            | Self::Vmin(v)
            | Self::Vmax(v)
            | Self::Ch(v)
            | Self::Dvw(v)
            | Self::Dvh(v)
            | Self::Svw(v)
            | Self::Svh(v)
            | Self::Lvw(v)
            | Self::Lvh(v)
            | Self::Cqw(v)
            | Self::Cqh(v) => v.get(),
        }
    }

    /// Attempt to transform the value while preserving its unit and the finite-value invariant.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidCssNumber::NonFinite`] if `f` produces `NaN` or infinity.
    pub fn try_map_value(self, f: impl FnOnce(f64) -> f64) -> Result<Self, InvalidCssNumber> {
        let mapped = FiniteF64::try_new(f(self.value()))?;
        Ok(match self {
            Self::Px(_) => Self::Px(mapped),
            Self::Em(_) => Self::Em(mapped),
            Self::Rem(_) => Self::Rem(mapped),
            Self::Vw(_) => Self::Vw(mapped),
            Self::Vh(_) => Self::Vh(mapped),
            Self::Vmin(_) => Self::Vmin(mapped),
            Self::Vmax(_) => Self::Vmax(mapped),
            Self::Ch(_) => Self::Ch(mapped),
            Self::Dvw(_) => Self::Dvw(mapped),
            Self::Dvh(_) => Self::Dvh(mapped),
            Self::Svw(_) => Self::Svw(mapped),
            Self::Svh(_) => Self::Svh(mapped),
            Self::Lvw(_) => Self::Lvw(mapped),
            Self::Lvh(_) => Self::Lvh(mapped),
            Self::Cqw(_) => Self::Cqw(mapped),
            Self::Cqh(_) => Self::Cqh(mapped),
        })
    }

    /// Transform the value while preserving its unit.
    ///
    /// # Panics
    ///
    /// Panics if `f` produces `NaN` or infinity. Use [`CssLength::try_map_value`] when the
    /// transformed value can be invalid at runtime.
    #[must_use]
    #[track_caller]
    pub fn map_value(self, f: impl FnOnce(f64) -> f64) -> Self {
        self.try_map_value(f)
            .unwrap_or_else(|error| panic!("{error}"))
    }
}

impl CssWriteTo for CssLength {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Px(v) => write!(w, "{v}px"),
            Self::Em(v) => write!(w, "{v}em"),
            Self::Rem(v) => write!(w, "{v}rem"),
            Self::Vw(v) => write!(w, "{v}vw"),
            Self::Vh(v) => write!(w, "{v}vh"),
            Self::Vmin(v) => write!(w, "{v}vmin"),
            Self::Vmax(v) => write!(w, "{v}vmax"),
            Self::Ch(v) => write!(w, "{v}ch"),
            Self::Dvw(v) => write!(w, "{v}dvw"),
            Self::Dvh(v) => write!(w, "{v}dvh"),
            Self::Svw(v) => write!(w, "{v}svw"),
            Self::Svh(v) => write!(w, "{v}svh"),
            Self::Lvw(v) => write!(w, "{v}lvw"),
            Self::Lvh(v) => write!(w, "{v}lvh"),
            Self::Cqw(v) => write!(w, "{v}cqw"),
            Self::Cqh(v) => write!(w, "{v}cqh"),
        }
    }
}

impl std::ops::Neg for CssLength {
    type Output = Self;

    fn neg(self) -> Self {
        self.map_value(|v| -v)
    }
}

impl_checked_value!(CssLength);
