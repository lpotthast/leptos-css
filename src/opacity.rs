use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{InvalidCssNumber, UnitInterval, serialization::CssWriteTo};

/// A normalized opacity value in the inclusive range `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Opacity(UnitInterval);

impl Opacity {
    /// Construct an opacity value.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self(UnitInterval::new(value))
    }

    /// Attempt to construct an opacity value.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        UnitInterval::try_new(value).map(Self)
    }
}

impl CssWriteTo for Opacity {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{}", self.0)
    }
}

impl_checked_value!(Opacity);
