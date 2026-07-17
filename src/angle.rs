use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{FiniteF64, serialization::CssWriteTo};

/// CSS angle values.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssAngle {
    /// Degrees (360deg = full circle).
    Deg(FiniteF64),
    /// Radians (2π rad = full circle).
    Rad(FiniteF64),
    /// Turns (1turn = full circle).
    Turn(FiniteF64),
    /// Gradians (400grad = full circle).
    Grad(FiniteF64),
}

impl CssWriteTo for CssAngle {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Deg(v) => write!(w, "{v}deg"),
            Self::Rad(v) => write!(w, "{v}rad"),
            Self::Turn(v) => write!(w, "{v}turn"),
            Self::Grad(v) => write!(w, "{v}grad"),
        }
    }
}

impl std::ops::Neg for CssAngle {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Deg(v) => Self::Deg(-v),
            Self::Rad(v) => Self::Rad(-v),
            Self::Turn(v) => Self::Turn(-v),
            Self::Grad(v) => Self::Grad(-v),
        }
    }
}

impl_checked_value!(CssAngle);
