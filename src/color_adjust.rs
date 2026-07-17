use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::serialization::CssWriteTo;

/// Values for the modern `forced-color-adjust` property.
///
/// [CSS Color Adjustment § 3.2]: https://www.w3.org/TR/css-color-adjust-1/#forced-color-adjust-prop
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ForcedColorAdjust {
    /// Allow the user agent to adjust colors in forced-colors mode.
    Auto,
    /// Prevent automatic color adjustment.
    None,
    /// Preserve the parent's used color while otherwise behaving like `none`.
    PreserveParentColor,
}

impl CssWriteTo for ForcedColorAdjust {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::PreserveParentColor => "preserve-parent-color",
        })
    }
}

/// Values for the modern `print-color-adjust` property.
///
/// [CSS Color Adjustment § 4.1]: https://www.w3.org/TR/css-color-adjust-1/#print-color-adjust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PrintColorAdjust {
    /// Allow output-device economy adjustments.
    Economy,
    /// Preserve the specified colors and styling where possible.
    Exact,
}

impl CssWriteTo for PrintColorAdjust {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::Economy => "economy",
            Self::Exact => "exact",
        })
    }
}

impl_checked_value!(ForcedColorAdjust, PrintColorAdjust);
