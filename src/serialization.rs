use std::fmt;

/// Open serialization trait for CSS-like values.
///
/// This powers both [`fmt::Display`]-style formatting and the zero-allocation
/// [`write_to`](CssWriteTo::write_to) method through a single implementation. Downstream types
/// may implement this trait for application-specific serialization, but doing so does not make a
/// type eligible for checked declarations; that capability is represented by the sealed
/// [`CheckedCssValue`](crate::CheckedCssValue) trait.
///
/// ```rust
/// use std::fmt;
/// use leptos_css::CssWriteTo;
///
/// struct ExternalValue(&'static str);
///
/// impl CssWriteTo for ExternalValue {
///     fn css_fmt<W: fmt::Write>(&self, output: &mut W) -> fmt::Result {
///         output.write_str(self.0)
///     }
/// }
///
/// let mut output = String::new();
/// ExternalValue("application-specific").write_to(&mut output);
/// assert_eq!(output, "application-specific");
/// ```
///
/// Serialization alone cannot enter the checked custom-property API:
///
/// ```compile_fail
/// use std::fmt;
/// use leptos_css::{CssCustomProperty, CssWriteTo};
///
/// struct Raw(&'static str);
///
/// impl CssWriteTo for Raw {
///     fn css_fmt<W: fmt::Write>(&self, output: &mut W) -> fmt::Result {
///         output.write_str(self.0)
///     }
/// }
///
/// let property = CssCustomProperty::<Raw>::new("--unsafe");
/// let _ = property.declare(Raw("red;color:lime"));
/// ```
pub trait CssWriteTo {
    /// Write the CSS representation to a `fmt::Write` target.
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;

    /// Write the CSS representation directly to a `String` buffer.
    ///
    /// This is infallible because `fmt::Write` for `String` never fails.
    fn write_to(&self, buf: &mut String) {
        let _ = self.css_fmt(buf);
    }
}
