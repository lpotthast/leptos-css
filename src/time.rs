use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{FiniteF64, serialization::CssWriteTo};

/// CSS time values.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssTime {
    /// Seconds.
    S(FiniteF64),
    /// Milliseconds.
    Ms(FiniteF64),
}

impl CssWriteTo for CssTime {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::S(v) => write!(w, "{v}s"),
            Self::Ms(v) => write!(w, "{v}ms"),
        }
    }
}

impl std::ops::Neg for CssTime {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::S(v) => Self::S(-v),
            Self::Ms(v) => Self::Ms(-v),
        }
    }
}

impl_checked_value!(CssTime);
