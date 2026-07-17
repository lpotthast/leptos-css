use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{
    FiniteF64, InvalidCssNumber, PercentageChannel, UnitInterval,
    number::{assert_finite, try_finite},
    serialization::CssWriteTo,
};

/// CSS named-color keywords supported by the typed API.
///
/// This deliberately uses an enum instead of accepting arbitrary strings. More names can be
/// added without weakening the invariant that every represented value is a valid `<color>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CssColorName {
    /// `transparent`.
    Transparent,
    /// `currentcolor`.
    CurrentColor,
    /// `black`.
    Black,
    /// `silver`.
    Silver,
    /// `gray`.
    Gray,
    /// `white`.
    White,
    /// `maroon`.
    Maroon,
    /// `red`.
    Red,
    /// `purple`.
    Purple,
    /// `fuchsia`.
    Fuchsia,
    /// `green`.
    Green,
    /// `lime`.
    Lime,
    /// `olive`.
    Olive,
    /// `yellow`.
    Yellow,
    /// `navy`.
    Navy,
    /// `blue`.
    Blue,
    /// `teal`.
    Teal,
    /// `aqua`.
    Aqua,
}

impl CssColorName {
    /// Return the CSS keyword.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transparent => "transparent",
            Self::CurrentColor => "currentcolor",
            Self::Black => "black",
            Self::Silver => "silver",
            Self::Gray => "gray",
            Self::White => "white",
            Self::Maroon => "maroon",
            Self::Red => "red",
            Self::Purple => "purple",
            Self::Fuchsia => "fuchsia",
            Self::Green => "green",
            Self::Lime => "lime",
            Self::Olive => "olive",
            Self::Yellow => "yellow",
            Self::Navy => "navy",
            Self::Blue => "blue",
            Self::Teal => "teal",
            Self::Aqua => "aqua",
        }
    }
}

/// CSS color values from [CSS Color Level 4].
///
/// [CSS Color Level 4]: https://www.w3.org/TR/css-color-4/#color-syntax
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssColor {
    /// RGB color (e.g., `rgb(255, 128, 0)`).
    Rgb(u8, u8, u8),
    /// RGBA color with alpha (e.g., `rgba(255, 128, 0, 0.5)`).
    Rgba(u8, u8, u8, UnitInterval),
    /// HSL color (e.g., `hsl(120, 100%, 50%)`).
    /// Hue in degrees (any finite value; CSS wraps it around the 0-360 degree color wheel),
    /// saturation and lightness as percentages (0-100).
    Hsl(FiniteF64, PercentageChannel, PercentageChannel),
    /// HSLA color with alpha (e.g., `hsla(120, 100%, 50%, 0.5)`).
    /// Hue in degrees (any finite value; CSS wraps it around the 0-360 degree color wheel),
    /// saturation and lightness as percentages (0-100), alpha (0.0-1.0).
    Hsla(
        FiniteF64,
        PercentageChannel,
        PercentageChannel,
        UnitInterval,
    ),
    /// A CSS named color.
    Named(CssColorName),
}

impl CssWriteTo for CssColor {
    #[allow(clippy::many_single_char_names)]
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Rgb(r, g, b) => write!(w, "rgb({r}, {g}, {b})"),
            Self::Rgba(r, g, b, a) => write!(w, "rgba({r}, {g}, {b}, {a})"),
            Self::Hsl(h, s, l) => write!(w, "hsl({h}, {s}%, {l}%)"),
            Self::Hsla(h, s, l, a) => write!(w, "hsla({h}, {s}%, {l}%, {a})"),
            Self::Named(name) => w.write_str(name.as_str()),
        }
    }
}

/// Create a CSS RGB color value.
pub const fn rgb(r: u8, g: u8, b: u8) -> CssColor {
    CssColor::Rgb(r, g, b)
}

/// Create a CSS RGBA color value with alpha.
///
/// # Panics
///
/// Panics if `a` is `NaN`, infinite, or outside the inclusive range `[0.0, 1.0]`.
pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> CssColor {
    CssColor::Rgba(r, g, b, UnitInterval::new(a))
}

/// Attempt to create a CSS RGBA color value with alpha.
///
/// # Errors
///
/// Returns an error if `a` is `NaN`, infinite, or outside the inclusive range `[0.0, 1.0]`.
pub fn try_rgba(r: u8, g: u8, b: u8, a: f64) -> Result<CssColor, InvalidCssNumber> {
    UnitInterval::try_new(a).map(|a| CssColor::Rgba(r, g, b, a))
}

/// Create a CSS HSL color value.
///
/// * `h` - Any finite hue in degrees; CSS wraps it around the 0-360 degree color wheel.
/// * `s` - Saturation as a percentage (0-100).
/// * `l` - Lightness as a percentage (0-100).
///
/// # Panics
///
/// Panics if `h` is `NaN` or infinite, or if `s` or `l` is outside the inclusive range
/// `[0.0, 100.0]`.
pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    CssColor::Hsl(
        assert_finite(h, "hsl hue"),
        PercentageChannel::new(s),
        PercentageChannel::new(l),
    )
}

/// Attempt to create a CSS HSL color value.
///
/// # Errors
///
/// Returns an error if `h` is non-finite or if `s` or `l` is non-finite or outside the
/// inclusive range `[0.0, 100.0]`.
pub fn try_hsl(
    h: impl Into<f64>,
    s: impl Into<f64>,
    l: impl Into<f64>,
) -> Result<CssColor, InvalidCssNumber> {
    let h = try_finite(h.into(), "hsl hue")?;
    let s = PercentageChannel::try_new(s.into())?;
    let l = PercentageChannel::try_new(l.into())?;
    Ok(CssColor::Hsl(h, s, l))
}

/// Create a CSS HSLA color value with alpha.
///
/// * `h` - Any finite hue in degrees; CSS wraps it around the 0-360 degree color wheel.
/// * `s` - Saturation as a percentage (0-100).
/// * `l` - Lightness as a percentage (0-100).
/// * `a` - Alpha (0.0-1.0).
///
/// # Panics
///
/// Panics if `h` is `NaN` or infinite, if `s` or `l` is outside `[0.0, 100.0]`, or if `a`
/// is outside `[0.0, 1.0]`.
pub fn hsla(
    h: impl Into<f64>,
    s: impl Into<f64>,
    l: impl Into<f64>,
    a: impl Into<f64>,
) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    let a = a.into();
    CssColor::Hsla(
        assert_finite(h, "hsla hue"),
        PercentageChannel::new(s),
        PercentageChannel::new(l),
        UnitInterval::new(a),
    )
}

/// Attempt to create a CSS HSLA color value.
///
/// # Errors
///
/// Returns an error if `h` is non-finite, or if `s`, `l`, or `a` is non-finite or outside
/// its documented inclusive range.
pub fn try_hsla(
    h: impl Into<f64>,
    s: impl Into<f64>,
    l: impl Into<f64>,
    a: impl Into<f64>,
) -> Result<CssColor, InvalidCssNumber> {
    let h = try_finite(h.into(), "hsla hue")?;
    let s = PercentageChannel::try_new(s.into())?;
    let l = PercentageChannel::try_new(l.into())?;
    let a = UnitInterval::try_new(a.into())?;
    Ok(CssColor::Hsla(h, s, l, a))
}

impl_checked_value!(CssColor);
