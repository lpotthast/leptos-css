use crate::serialization::CssWriteTo;

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A crate-validated CSS value grammar eligible for checked declarations.
///
/// This trait is sealed. Downstream types may implement [`CssWriteTo`] for serialization, but
/// only crate-owned closed grammars implement this trait.
///
/// ```compile_fail
/// use std::fmt;
/// use leptos_css::{CheckedCssValue, CssWriteTo};
///
/// struct ExternalValue;
///
/// impl CssWriteTo for ExternalValue {
///     fn css_fmt<W: fmt::Write>(&self, _output: &mut W) -> fmt::Result {
///         Ok(())
///     }
/// }
///
/// impl CheckedCssValue for ExternalValue {}
/// ```
pub trait CheckedCssValue: CssWriteTo + sealed::Sealed {}

impl<T> CheckedCssValue for T where T: CssWriteTo + sealed::Sealed {}

macro_rules! impl_checked_value {
    ($($ty:ty),+ $(,)?) => {$ (
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.css_fmt(f)
            }
        }

        impl $crate::checked_value::sealed::Sealed for $ty {}
    )+};
}

pub(crate) use impl_checked_value;
