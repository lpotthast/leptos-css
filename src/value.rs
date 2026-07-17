use std::{borrow::Cow, fmt, marker::PhantomData};

/// Trait for CSS types that can write their representation to any `fmt::Write` target.
///
/// This powers both [`fmt::Display`]-style formatting and the zero-allocation
/// [`write_to`](CssWriteTo::write_to) method through a single implementation.
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

macro_rules! impl_display_via_css_fmt {
    ($($ty:ty),+ $(,)?) => {$(
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.css_fmt(f)
            }
        }
    )+};
}

/// Error returned when a custom-property name is outside the checked identifier subset.
///
/// Checked names must start with `--`, followed by an ASCII letter or underscore, and then only
/// contain ASCII letters, digits, hyphens, or underscores. CSS accepts additional escaped and
/// non-ASCII identifiers, but rejecting those forms keeps this API fail-closed without accepting
/// an unchecked token string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidCustomPropertyName {
    name: String,
}

impl InvalidCustomPropertyName {
    /// Return the rejected name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for InvalidCustomPropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "custom property name must match --[A-Za-z_][A-Za-z0-9_-]*, got {:?}",
            self.name
        )
    }
}

impl std::error::Error for InvalidCustomPropertyName {}

const fn is_ascii_letter(byte: u8) -> bool {
    (byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')
}

const fn is_checked_identifier(identifier: &str) -> bool {
    let bytes = identifier.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !is_ascii_letter(bytes[0]) && bytes[0] != b'_' {
        return false;
    }
    let mut index = 1;
    while index < bytes.len() {
        let byte = bytes[index];
        if !(is_ascii_letter(byte)
            || (byte >= b'0' && byte <= b'9')
            || byte == b'-'
            || byte == b'_')
        {
            return false;
        }
        index += 1;
    }
    true
}

const fn is_checked_custom_property_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes[0] != b'-' || bytes[1] != b'-' {
        return false;
    }
    if !is_ascii_letter(bytes[2]) && bytes[2] != b'_' {
        return false;
    }
    let mut index = 3;
    while index < bytes.len() {
        let byte = bytes[index];
        if !(is_ascii_letter(byte)
            || (byte >= b'0' && byte <= b'9')
            || byte == b'-'
            || byte == b'_')
        {
            return false;
        }
        index += 1;
    }
    true
}

/// A validated CSS custom property associated with one Rust value grammar.
///
/// The type parameter prevents ordinary checked code from using, for example, a color variable
/// as a size. The same CSS name must not be independently constructed with two different Rust
/// grammar types; conflicting handwritten or external declarations are outside this crate's
/// checked boundary.
///
/// [CSS Custom Properties Level 1]: https://www.w3.org/TR/css-variables-1/
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CssCustomProperty<T> {
    name: Cow<'static, str>,
    marker: PhantomData<fn() -> T>,
}

impl<T> Clone for CssCustomProperty<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> CssCustomProperty<T> {
    /// Construct a checked custom property from a static name.
    ///
    /// This constructor is const and therefore suitable for module-level declarations. Prefer the
    /// [`css_custom_property`](crate::css_custom_property) macro for compile-time-checked literals.
    ///
    /// # Panics
    ///
    /// Const evaluation fails if `name` is outside the checked identifier subset.
    #[track_caller]
    pub const fn from_static(name: &'static str) -> Self {
        assert!(
            is_checked_custom_property_name(name),
            "invalid checked CSS custom-property name"
        );
        Self {
            name: Cow::Borrowed(name),
            marker: PhantomData,
        }
    }

    /// Construct a checked custom property.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not in the checked custom-property identifier subset. Use
    /// [`CssCustomProperty::try_new`] for runtime-derived names.
    #[track_caller]
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self::try_new(name).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a checked custom property.
    pub fn try_new(name: impl Into<Cow<'static, str>>) -> Result<Self, InvalidCustomPropertyName> {
        let name = name.into();
        if is_checked_custom_property_name(&name) {
            Ok(Self {
                name,
                marker: PhantomData,
            })
        } else {
            Err(InvalidCustomPropertyName {
                name: name.into_owned(),
            })
        }
    }

    /// Return the validated, case-sensitive CSS custom-property name.
    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub(crate) fn cloned_name(&self) -> Cow<'static, str> {
        self.name.clone()
    }
}

/// Declare a reusable, statically named typed CSS custom property.
///
/// The name is checked during const evaluation and the resulting value carries the declared Rust
/// grammar type.
///
/// ```rust
/// use leptos_css::{CssColor, css_custom_property};
///
/// css_custom_property!(ACCENT_COLOR: CssColor = "--accent-color");
/// assert_eq!(ACCENT_COLOR.as_str(), "--accent-color");
/// ```
///
/// Invalid literals fail during const evaluation:
///
/// ```compile_fail
/// use leptos_css::{CssColor, css_custom_property};
///
/// css_custom_property!(ACCENT_COLOR: CssColor = "accent-color");
/// ```
#[macro_export]
macro_rules! css_custom_property {
    ($(#[$meta:meta])* $vis:vis $name:ident : $value:ty = $css_name:literal $(;)?) => {
        $(#[$meta])*
        $vis const $name: $crate::CssCustomProperty<$value> =
            $crate::CssCustomProperty::from_static($css_name);
    };
}

impl<T> fmt::Display for CssCustomProperty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A checked literal or a typed custom-property reference for one CSS grammar.
///
/// The representation is private so callers cannot construct an untyped variable reference or
/// raw fallback token stream.
#[derive(Clone, Debug, PartialEq)]
pub struct DeclarationValue<T>(DeclarationValueKind<T>);

#[derive(Clone, Debug, PartialEq)]
enum DeclarationValueKind<T> {
    Literal(T),
    Variable(CssVariableReference<T>),
}

impl<T> From<T> for DeclarationValue<T> {
    fn from(value: T) -> Self {
        Self(DeclarationValueKind::Literal(value))
    }
}

/// A typed `var()` reference with a mandatory, grammar-matching fallback.
#[derive(Clone, Debug, PartialEq)]
pub struct CssVariableReference<T> {
    property: CssCustomProperty<T>,
    fallback: Box<DeclarationValue<T>>,
}

impl<T> From<CssVariableReference<T>> for DeclarationValue<T> {
    fn from(value: CssVariableReference<T>) -> Self {
        Self(DeclarationValueKind::Variable(value))
    }
}

impl<T: CssWriteTo> CssWriteTo for DeclarationValue<T> {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match &self.0 {
            DeclarationValueKind::Literal(value) => value.css_fmt(w),
            DeclarationValueKind::Variable(value) => value.css_fmt(w),
        }
    }
}

impl<T: CssWriteTo> CssWriteTo for CssVariableReference<T> {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "var({}, ", self.property)?;
        self.fallback.css_fmt(w)?;
        w.write_char(')')
    }
}

impl<T: CssWriteTo> fmt::Display for DeclarationValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.css_fmt(f)
    }
}

impl<T: CssWriteTo> fmt::Display for CssVariableReference<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.css_fmt(f)
    }
}

/// Reference a typed custom property using a mandatory fallback of the same grammar.
///
/// Requiring a fallback prevents an undefined custom property from invalidating the consuming
/// declaration at computed-value time. The fallback may itself be another typed `var()` result.
///
/// [CSS variable substitution]: https://www.w3.org/TR/css-variables-1/#using-variables
pub fn var<T>(
    property: &CssCustomProperty<T>,
    fallback: impl Into<DeclarationValue<T>>,
) -> CssVariableReference<T> {
    CssVariableReference {
        property: property.clone(),
        fallback: Box::new(fallback.into()),
    }
}

/// Error returned when a floating-point value cannot be represented by a typed CSS value.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum InvalidCssNumber {
    /// The value is `NaN` or positive/negative infinity.
    NonFinite {
        /// The rejected value.
        value: f64,
        /// The CSS unit or channel being constructed.
        context: &'static str,
    },
    /// The value is finite but outside the permitted inclusive range.
    OutOfRange {
        /// The rejected value.
        value: f64,
        /// The CSS unit or channel being constructed.
        context: &'static str,
        /// The inclusive lower bound.
        min: f64,
        /// The inclusive upper bound.
        max: f64,
    },
    /// The value is finite but negative where CSS requires a non-negative number.
    Negative {
        /// The rejected value.
        value: f64,
        /// The CSS unit or channel being constructed.
        context: &'static str,
    },
}

impl fmt::Display for InvalidCssNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { value, context } => {
                write!(f, "CSS {context} value must be finite, got {value}")
            }
            Self::OutOfRange {
                value,
                context,
                min,
                max,
            } => write!(
                f,
                "CSS {context} must be in range [{min}, {max}], got {value}"
            ),
            Self::Negative { value, context } => {
                write!(f, "CSS {context} must be non-negative, got {value}")
            }
        }
    }
}

impl std::error::Error for InvalidCssNumber {}

/// A finite IEEE-754 double used by typed CSS numeric values.
///
/// The private field makes `NaN` and positive/negative infinity unrepresentable through the
/// public API. Use [`FiniteF64::try_new`] for runtime-derived values and [`FiniteF64::new`] for
/// values whose validity is a programming invariant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteF64(f64);

const fn try_finite(value: f64, context: &'static str) -> Result<FiniteF64, InvalidCssNumber> {
    if value.is_finite() {
        Ok(FiniteF64(value))
    } else {
        Err(InvalidCssNumber::NonFinite { value, context })
    }
}

impl FiniteF64 {
    /// Construct a finite value, panicking if `value` is `NaN` or infinite.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not finite.
    #[track_caller]
    pub const fn new(value: f64) -> Self {
        assert!(value.is_finite(), "CSS numeric value must be finite");
        Self(value)
    }

    /// Attempt to construct a finite value.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidCssNumber::NonFinite`] if `value` is `NaN` or infinite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_finite(value, "number")
    }

    /// Return the contained finite `f64`.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for FiniteF64 {
    type Error = InvalidCssNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<FiniteF64> for f64 {
    fn from(value: FiniteF64) -> Self {
        value.get()
    }
}

impl fmt::Display for FiniteF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Neg for FiniteF64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

/// A finite, non-negative number used by CSS grammars with a lower bound of zero.
///
/// The inner value is private, so negative and non-finite values cannot be represented.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonNegativeFiniteF64(FiniteF64);

impl NonNegativeFiniteF64 {
    /// Construct a non-negative finite number.
    ///
    /// # Panics
    ///
    /// Panics if `value` is negative, `NaN`, or infinite.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a non-negative finite number.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_non_negative(value, "non-negative number").map(Self)
    }

    /// Return the contained number.
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl fmt::Display for NonNegativeFiniteF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A finite number in the inclusive range `0..=1`.
///
/// This is the range used by alpha channels and normalized opacity values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitInterval(FiniteF64);

impl UnitInterval {
    /// Construct a value in the inclusive range `0..=1`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside the range or is not finite.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a value in the inclusive range `0..=1`.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_in_range(value, "unit interval", 0.0, 1.0).map(Self)
    }

    /// Return the contained number.
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl fmt::Display for UnitInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A finite percentage channel in the inclusive range `0..=100`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PercentageChannel(FiniteF64);

impl PercentageChannel {
    /// Construct a percentage channel in the inclusive range `0..=100`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside the range or is not finite.
    #[track_caller]
    pub fn new(value: f64) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a percentage channel in the inclusive range `0..=100`.
    pub fn try_new(value: f64) -> Result<Self, InvalidCssNumber> {
        try_in_range(value, "percentage channel", 0.0, 100.0).map(Self)
    }

    /// Return the contained percentage.
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl fmt::Display for PercentageChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

/// CSS-wide keywords accepted by ordinary CSS properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GlobalKeyword {
    /// The `inherit` keyword.
    Inherit,
    /// The `initial` keyword.
    Initial,
    /// The `unset` keyword.
    Unset,
    /// The `revert` keyword.
    Revert,
    /// The `revert-layer` keyword.
    RevertLayer,
}

impl CssWriteTo for GlobalKeyword {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Unset => "unset",
            Self::Revert => "revert",
            Self::RevertLayer => "revert-layer",
        })
    }
}

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

// --- CSS shorthand value types ---

/// CSS font-weight values.
///
/// Represents the `font-weight` CSS property as a typed enum rather than a raw string.
/// Numeric weights range from 100 (thinnest) to 900 (boldest), with keyword aliases
/// for common values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FontWeight {
    /// Weight 100 — Thin / Hairline.
    W100,
    /// Weight 200 — Extra Light / Ultra Light.
    W200,
    /// Weight 300 — Light.
    W300,
    /// Weight 400 — Normal / Regular (same as `WNormal`).
    W400,
    /// Weight 500 — Medium.
    W500,
    /// Weight 600 — Semi Bold / Demi Bold.
    W600,
    /// Weight 700 — Bold (same as `WBold`).
    W700,
    /// Weight 800 — Extra Bold / Ultra Bold.
    W800,
    /// Weight 900 — Black / Heavy.
    W900,
    /// Keyword `lighter` — one relative step lighter than the inherited weight.
    WLighter,
    /// Keyword `normal` — equivalent to weight 400.
    WNormal,
    /// Keyword `bold` — equivalent to weight 700.
    WBold,
    /// Keyword `bolder` — one relative step bolder than the inherited weight.
    WBolder,
}

impl CssWriteTo for FontWeight {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::W100 => w.write_str("100"),
            Self::W200 => w.write_str("200"),
            Self::W300 => w.write_str("300"),
            Self::W400 => w.write_str("400"),
            Self::W500 => w.write_str("500"),
            Self::W600 => w.write_str("600"),
            Self::W700 => w.write_str("700"),
            Self::W800 => w.write_str("800"),
            Self::W900 => w.write_str("900"),
            Self::WLighter => w.write_str("lighter"),
            Self::WNormal => w.write_str("normal"),
            Self::WBold => w.write_str("bold"),
            Self::WBolder => w.write_str("bolder"),
        }
    }
}

macro_rules! box_model_shorthand {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            /// Only on the top edge (other edges are `0`).
            Top($value),
            /// Only on the right edge (other edges are `0`).
            Right($value),
            /// Only on the bottom edge (other edges are `0`).
            Bottom($value),
            /// Only on the left edge (other edges are `0`).
            Left($value),
            /// Equal on all four edges.
            All($value),
            /// Separate vertical (top/bottom) and horizontal (left/right).
            Double($value, $value),
            /// Explicit for each edge: top, right, bottom, left.
            Full($value, $value, $value, $value),
        }

        impl CssWriteTo for $name {
            fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    Self::Top(size) => {
                        size.css_fmt(w)?;
                        w.write_str(" 0 0 0")
                    }
                    Self::Right(size) => {
                        w.write_str("0 ")?;
                        size.css_fmt(w)?;
                        w.write_str(" 0 0")
                    }
                    Self::Bottom(size) => {
                        w.write_str("0 0 ")?;
                        size.css_fmt(w)?;
                        w.write_str(" 0")
                    }
                    Self::Left(size) => {
                        w.write_str("0 0 0 ")?;
                        size.css_fmt(w)
                    }
                    Self::All(size) => size.css_fmt(w),
                    Self::Double(vertical, horizontal) => {
                        vertical.css_fmt(w)?;
                        w.write_char(' ')?;
                        horizontal.css_fmt(w)
                    }
                    Self::Full(top, right, bottom, left) => {
                        top.css_fmt(w)?;
                        w.write_char(' ')?;
                        right.css_fmt(w)?;
                        w.write_char(' ')?;
                        bottom.css_fmt(w)?;
                        w.write_char(' ')?;
                        left.css_fmt(w)
                    }
                }
            }
        }

    };
}

box_model_shorthand!(
    /// CSS margin shorthand values.
    ///
    /// Represents the `margin` CSS shorthand as a typed enum. Each variant maps to
    /// a different shorthand form:
    ///
    /// - `All(dim)` → `margin: dim`
    /// - `Double(v, h)` → `margin: v h`
    /// - `Full(t, r, b, l)` → `margin: t r b l`
    /// - `Top(dim)` → `margin: dim 0 0 0` (and similarly for other sides)
    ///
    /// [CSS Box Model margin grammar]: https://www.w3.org/TR/css-box-3/#margin-physical
    Margin,
    LengthPercentageAuto
);

box_model_shorthand!(
    /// CSS `inset` shorthand values.
    ///
    /// Each component accepts `auto`, a direct length-percentage, or a typed calculation.
    ///
    /// [CSS Positioned Layout inset grammar]: https://www.w3.org/TR/css-position-3/#insets
    Inset,
    LengthPercentageAuto
);

/// A non-negative `<length-percentage>` suitable for grammars such as `padding` and `gap`.
///
/// `auto`, negative values, and non-finite values cannot be represented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonNegativeLengthPercentage(CssDimension);

impl NonNegativeLengthPercentage {
    /// Construct a non-negative length-percentage.
    ///
    /// # Panics
    ///
    /// Panics if `value` is negative.
    #[track_caller]
    pub fn new(value: CssDimension) -> Self {
        Self::try_from(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Return the underlying typed dimension.
    pub const fn get(self) -> CssDimension {
        self.0
    }
}

/// Error returned when a dimension is not a non-negative `<length-percentage>`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum InvalidNonNegativeLengthPercentage {
    /// The numeric value is negative.
    Negative(f64),
}

impl fmt::Display for InvalidNonNegativeLengthPercentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative(value) => write!(f, "CSS value must be non-negative, got {value}"),
        }
    }
}

impl std::error::Error for InvalidNonNegativeLengthPercentage {}

impl TryFrom<CssDimension> for NonNegativeLengthPercentage {
    type Error = InvalidNonNegativeLengthPercentage;

    fn try_from(value: CssDimension) -> Result<Self, Self::Error> {
        let number = match value {
            CssDimension::Length(length) => length.value(),
            CssDimension::Percent(number) => number.get(),
            CssDimension::Zero => 0.0,
        };
        if number < 0.0 {
            Err(InvalidNonNegativeLengthPercentage::Negative(number))
        } else {
            Ok(Self(value))
        }
    }
}

impl CssWriteTo for NonNegativeLengthPercentage {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.0.css_fmt(w)
    }
}

/// A direct or calculated `<length-percentage [0,∞]>` value.
///
/// Direct values are checked before construction. Calculations stay inside a CSS math function so
/// the specification's top-level range clamping applies after evaluation and substitution.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum NonNegativeLengthPercentageValue {
    /// A directly non-negative length or percentage.
    Direct(NonNegativeLengthPercentage),
    /// A typed calculation whose result is clamped to the property's non-negative range.
    Calculation(LengthPercentageCalculation),
}

impl From<NonNegativeLengthPercentage> for NonNegativeLengthPercentageValue {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::Direct(value)
    }
}

impl From<LengthPercentageCalculation> for NonNegativeLengthPercentageValue {
    fn from(value: LengthPercentageCalculation) -> Self {
        Self::Calculation(value)
    }
}

impl From<NonNegativeLengthPercentage> for DeclarationValue<NonNegativeLengthPercentageValue> {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        NonNegativeLengthPercentageValue::from(value).into()
    }
}

impl From<LengthPercentageCalculation> for DeclarationValue<NonNegativeLengthPercentageValue> {
    fn from(value: LengthPercentageCalculation) -> Self {
        NonNegativeLengthPercentageValue::from(value).into()
    }
}

impl CssWriteTo for NonNegativeLengthPercentageValue {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Direct(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
        }
    }
}

/// A logical or physical corner-radius longhand value.
///
/// The grammar is `<length-percentage [0,∞]>{1,2}`: one value creates a circular radius and two
/// values specify the horizontal and vertical radii of an ellipse.
///
/// [CSS Logical Properties § 4.6]: https://www.w3.org/TR/css-logical-1/#border-radius-properties
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum BorderCornerRadius {
    /// One radius used in both dimensions.
    Circular(NonNegativeLengthPercentageValue),
    /// Separate horizontal and vertical radii.
    Elliptical(
        NonNegativeLengthPercentageValue,
        NonNegativeLengthPercentageValue,
    ),
}

impl BorderCornerRadius {
    /// Construct a circular radius from a direct non-negative value.
    pub fn circular(value: CssDimension) -> Self {
        Self::Circular(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct an elliptical radius from direct non-negative values.
    pub fn elliptical(horizontal: CssDimension, vertical: CssDimension) -> Self {
        Self::Elliptical(
            NonNegativeLengthPercentage::new(horizontal).into(),
            NonNegativeLengthPercentage::new(vertical).into(),
        )
    }

    /// Construct a circular radius from a typed calculation.
    pub fn calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::Circular(LengthPercentageCalculation::new(expression).into())
    }
}

impl CssWriteTo for BorderCornerRadius {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Circular(radius) => radius.css_fmt(w),
            Self::Elliptical(horizontal, vertical) => {
                horizontal.css_fmt(w)?;
                w.write_char(' ')?;
                vertical.css_fmt(w)
            }
        }
    }
}

box_model_shorthand!(
    /// CSS padding shorthand values.
    ///
    /// Mirrors the [`Margin`] type but for the `padding` property.
    ///
    /// - `All(dim)` → `padding: dim`
    /// - `Double(v, h)` → `padding: v h`
    /// - `Full(t, r, b, l)` → `padding: t r b l`
    /// - `Top(dim)` → `padding: dim 0 0 0` (and similarly for other sides)
    ///
    /// [CSS Box Model padding grammar]: https://www.w3.org/TR/css-box-3/#padding-physical
    Padding,
    NonNegativeLengthPercentageValue
);

impl Padding {
    /// Construct equal padding on all four edges.
    #[track_caller]
    pub fn all(value: CssDimension) -> Self {
        Self::All(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct vertical and horizontal padding.
    #[track_caller]
    pub fn double(vertical: CssDimension, horizontal: CssDimension) -> Self {
        Self::Double(
            NonNegativeLengthPercentage::new(vertical).into(),
            NonNegativeLengthPercentage::new(horizontal).into(),
        )
    }

    /// Construct explicit top, right, bottom, and left padding.
    #[track_caller]
    pub fn full(
        top: CssDimension,
        right: CssDimension,
        bottom: CssDimension,
        left: CssDimension,
    ) -> Self {
        Self::Full(
            NonNegativeLengthPercentage::new(top).into(),
            NonNegativeLengthPercentage::new(right).into(),
            NonNegativeLengthPercentage::new(bottom).into(),
            NonNegativeLengthPercentage::new(left).into(),
        )
    }

    /// Construct equal calculated padding on all four edges.
    pub fn calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::All(LengthPercentageCalculation::new(expression).into())
    }
}

macro_rules! axis_shorthand {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            /// One value for both the start and end edges.
            One($value),
            /// Separate start-edge and end-edge values.
            StartEnd($value, $value),
        }

        impl CssWriteTo for $name {
            fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    Self::One(value) => value.css_fmt(w),
                    Self::StartEnd(start, end) => {
                        start.css_fmt(w)?;
                        w.write_char(' ')?;
                        end.css_fmt(w)
                    }
                }
            }
        }
    };
}

axis_shorthand!(
    /// Values for the `padding-block` and `padding-inline` shorthands.
    PaddingAxis,
    NonNegativeLengthPercentageValue
);

impl PaddingAxis {
    /// Construct equal direct padding on both axis edges.
    pub fn one(value: CssDimension) -> Self {
        Self::One(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct separate direct padding on the start and end edges.
    pub fn start_end(start: CssDimension, end: CssDimension) -> Self {
        Self::StartEnd(
            NonNegativeLengthPercentage::new(start).into(),
            NonNegativeLengthPercentage::new(end).into(),
        )
    }

    /// Construct equal calculated padding on both axis edges.
    pub fn calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::One(LengthPercentageCalculation::new(expression).into())
    }
}

axis_shorthand!(
    /// Values for the `margin-block` and `margin-inline` shorthands.
    MarginAxis,
    LengthPercentageAuto
);

axis_shorthand!(
    /// Values for the `inset-block` and `inset-inline` shorthands.
    InsetAxis,
    LengthPercentageAuto
);

/// One component of the `gap`, `row-gap`, or `column-gap` grammar.
///
/// [CSS Box Alignment gap grammar]: https://www.w3.org/TR/css-align-3/#column-row-gap
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum GapValue {
    /// The `normal` keyword.
    Normal,
    /// A non-negative length or percentage.
    LengthPercentage(NonNegativeLengthPercentageValue),
}

impl From<NonNegativeLengthPercentage> for GapValue {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::LengthPercentage(value.into())
    }
}

impl From<LengthPercentageCalculation> for GapValue {
    fn from(value: LengthPercentageCalculation) -> Self {
        Self::LengthPercentage(value.into())
    }
}

impl CssWriteTo for GapValue {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Normal => w.write_str("normal"),
            Self::LengthPercentage(value) => value.css_fmt(w),
        }
    }
}

/// A checked `gap` shorthand value.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Gap {
    /// One value for both row and column gaps.
    One(GapValue),
    /// Separate row and column gaps.
    RowColumn(GapValue, GapValue),
}

impl CssWriteTo for Gap {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::One(value) => value.css_fmt(w),
            Self::RowColumn(row, column) => {
                row.css_fmt(w)?;
                w.write_char(' ')?;
                column.css_fmt(w)
            }
        }
    }
}

/// Values supported by the typed `width`, `height`, and minimum-size property APIs.
///
/// The `stretch` extension follows CSS Sizing Level 4; the remaining variants are defined by
/// CSS Sizing Level 3.
///
/// [CSS Sizing Level 3]: https://www.w3.org/TR/css-sizing-3/#sizing-values
/// [CSS Sizing Level 4]: https://www.w3.org/TR/css-sizing-4/#sizing-values
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Size {
    /// The `auto` keyword.
    Auto,
    /// A non-negative length or percentage.
    LengthPercentage(NonNegativeLengthPercentage),
    /// A typed length-percentage expression.
    Calculation(LengthPercentageCalculation),
    /// The `min-content` keyword.
    MinContent,
    /// The `max-content` keyword.
    MaxContent,
    /// The `fit-content()` function with a non-negative argument.
    FitContent(NonNegativeLengthPercentageValue),
    /// The `stretch` keyword.
    Stretch,
}

impl From<NonNegativeLengthPercentage> for Size {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::LengthPercentage(value)
    }
}

impl Size {
    /// Construct `fit-content()` with a directly non-negative argument.
    pub fn fit_content(value: CssDimension) -> Self {
        Self::FitContent(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct `fit-content()` with a typed calculation.
    pub fn fit_content_calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::FitContent(LengthPercentageCalculation::new(expression).into())
    }
}

impl CssWriteTo for Size {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::LengthPercentage(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
            Self::MinContent => w.write_str("min-content"),
            Self::MaxContent => w.write_str("max-content"),
            Self::FitContent(value) => {
                w.write_str("fit-content(")?;
                value.css_fmt(w)?;
                w.write_char(')')
            }
            Self::Stretch => w.write_str("stretch"),
        }
    }
}

/// Values supported by the typed maximum-size property APIs.
///
/// The `stretch` extension follows CSS Sizing Level 4.
///
/// [CSS Sizing Level 3]: https://www.w3.org/TR/css-sizing-3/#sizing-values
/// [CSS Sizing Level 4]: https://www.w3.org/TR/css-sizing-4/#sizing-values
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MaxSize {
    /// The `none` keyword.
    None,
    /// A non-negative length or percentage.
    LengthPercentage(NonNegativeLengthPercentage),
    /// A typed length-percentage expression.
    Calculation(LengthPercentageCalculation),
    /// The `min-content` keyword.
    MinContent,
    /// The `max-content` keyword.
    MaxContent,
    /// The `fit-content()` function with a non-negative argument.
    FitContent(NonNegativeLengthPercentageValue),
    /// The `stretch` keyword.
    Stretch,
}

impl From<NonNegativeLengthPercentage> for MaxSize {
    fn from(value: NonNegativeLengthPercentage) -> Self {
        Self::LengthPercentage(value)
    }
}

impl MaxSize {
    /// Construct `fit-content()` with a directly non-negative argument.
    pub fn fit_content(value: CssDimension) -> Self {
        Self::FitContent(NonNegativeLengthPercentage::new(value).into())
    }

    /// Construct `fit-content()` with a typed calculation.
    pub fn fit_content_calculated(expression: impl Into<CssDimensionExpr>) -> Self {
        Self::FitContent(LengthPercentageCalculation::new(expression).into())
    }
}

impl CssWriteTo for MaxSize {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::None => w.write_str("none"),
            Self::LengthPercentage(value) => value.css_fmt(w),
            Self::Calculation(value) => value.css_fmt(w),
            Self::MinContent => w.write_str("min-content"),
            Self::MaxContent => w.write_str("max-content"),
            Self::FitContent(value) => {
                w.write_str("fit-content(")?;
                value.css_fmt(w)?;
                w.write_char(')')
            }
            Self::Stretch => w.write_str("stretch"),
        }
    }
}

/// A standards-valid `touch-action` value.
///
/// [Pointer Events touch-action grammar]: https://www.w3.org/TR/pointerevents3/#the-touch-action-css-property
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TouchAction {
    /// `auto`.
    Auto,
    /// `none`.
    None,
    /// `manipulation`.
    Manipulation,
    /// `pan-x`.
    PanX,
    /// `pan-y`.
    PanY,
    /// `pinch-zoom`.
    PinchZoom,
    /// `pan-x pan-y`.
    PanXPanY,
    /// `pan-x pinch-zoom`.
    PanXPinchZoom,
    /// `pan-y pinch-zoom`.
    PanYPinchZoom,
    /// `pan-x pan-y pinch-zoom`.
    PanXPanYPinchZoom,
}

impl CssWriteTo for TouchAction {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Manipulation => "manipulation",
            Self::PanX => "pan-x",
            Self::PanY => "pan-y",
            Self::PinchZoom => "pinch-zoom",
            Self::PanXPanY => "pan-x pan-y",
            Self::PanXPinchZoom => "pan-x pinch-zoom",
            Self::PanYPinchZoom => "pan-y pinch-zoom",
            Self::PanXPanYPinchZoom => "pan-x pan-y pinch-zoom",
        })
    }
}

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

/// Error returned when a view-transition name is not in the checked `<custom-ident>` subset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidViewTransitionName {
    name: String,
}

impl InvalidViewTransitionName {
    /// Return the rejected identifier.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for InvalidViewTransitionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "view-transition-name must be a checked custom identifier other than none, auto, default, or a CSS-wide keyword, got {:?}",
            self.name
        )
    }
}

impl std::error::Error for InvalidViewTransitionName {}

fn is_reserved_custom_identifier(identifier: &str) -> bool {
    [
        "default",
        "inherit",
        "initial",
        "revert",
        "revert-layer",
        "unset",
    ]
    .iter()
    .any(|reserved| identifier.eq_ignore_ascii_case(reserved))
}

/// A checked `view-transition-name` value.
///
/// Names use a strict ASCII `<custom-ident>` subset and exclude the property-specific `none` and
/// `auto` keywords. The private representation prevents constructing a reserved identifier.
///
/// [CSS View Transitions § 2.1]: https://www.w3.org/TR/css-view-transitions-1/#view-transition-name-property
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewTransitionName(ViewTransitionNameValue);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ViewTransitionNameValue {
    None,
    Named(String),
}

impl ViewTransitionName {
    /// Construct the `none` keyword.
    pub const fn none() -> Self {
        Self(ViewTransitionNameValue::None)
    }

    /// Construct a checked named view transition.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not in the checked identifier subset or is reserved by CSS.
    #[track_caller]
    pub fn named(name: impl Into<String>) -> Self {
        Self::try_named(name).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to construct a checked named view transition.
    pub fn try_named(name: impl Into<String>) -> Result<Self, InvalidViewTransitionName> {
        let name = name.into();
        let property_reserved =
            name.eq_ignore_ascii_case("none") || name.eq_ignore_ascii_case("auto");
        if is_checked_identifier(&name)
            && !is_reserved_custom_identifier(&name)
            && !property_reserved
        {
            Ok(Self(ViewTransitionNameValue::Named(name)))
        } else {
            Err(InvalidViewTransitionName { name })
        }
    }
}

impl Default for ViewTransitionName {
    fn default() -> Self {
        Self::none()
    }
}

impl CssWriteTo for ViewTransitionName {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match &self.0 {
            ViewTransitionNameValue::None => w.write_str("none"),
            ViewTransitionNameValue::Named(name) => w.write_str(name),
        }
    }
}

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

/// A typed `z-index` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ZIndex {
    /// The `auto` keyword.
    Auto,
    /// An integer stack level.
    Integer(i32),
}

impl CssWriteTo for ZIndex {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::Integer(value) => write!(w, "{value}"),
        }
    }
}

impl_display_via_css_fmt!(
    CssLength,
    CssAngle,
    CssTime,
    CssColor,
    CssValue,
    CssDimension,
    LengthPercentageAuto,
    CssDimensionExpr,
    LengthPercentageCalculation,
    FontWeight,
    Margin,
    Inset,
    Padding,
    PaddingAxis,
    MarginAxis,
    InsetAxis,
    GapValue,
    Gap,
    NonNegativeLengthPercentage,
    NonNegativeLengthPercentageValue,
    BorderCornerRadius,
    Size,
    MaxSize,
    TouchAction,
    ForcedColorAdjust,
    PrintColorAdjust,
    ViewTransitionName,
    Opacity,
    ZIndex,
    GlobalKeyword,
);

// --- Neg impls ---

impl std::ops::Neg for CssLength {
    type Output = Self;

    fn neg(self) -> Self {
        self.map_value(|v| -v)
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

impl std::ops::Neg for CssTime {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::S(v) => Self::S(-v),
            Self::Ms(v) => Self::Ms(-v),
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

// --- Free convenience functions ---

// Length and percentage functions return CssDimension for type-safe use in dimension props.
// They also work in style closures via Into<CssValue>.

#[inline]
#[track_caller]
fn assert_finite(v: f64, context: &'static str) -> FiniteF64 {
    try_finite(v, context).unwrap_or_else(|error| panic!("{error}"))
}

#[inline]
fn try_in_range(
    v: f64,
    context: &'static str,
    min: f64,
    max: f64,
) -> Result<FiniteF64, InvalidCssNumber> {
    let finite = try_finite(v, context)?;
    if (min..=max).contains(&v) {
        Ok(finite)
    } else {
        Err(InvalidCssNumber::OutOfRange {
            value: v,
            context,
            min,
            max,
        })
    }
}

#[inline]
fn try_non_negative(v: f64, context: &'static str) -> Result<FiniteF64, InvalidCssNumber> {
    let finite = try_finite(v, context)?;
    if v >= 0.0 {
        Ok(finite)
    } else {
        Err(InvalidCssNumber::Negative { value: v, context })
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

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn finite(value: f64) -> FiniteF64 {
        FiniteF64::new(value)
    }

    #[test]
    fn test_css_length_write_to() {
        let cases = [
            (CssLength::Px(finite(10.0)), "10px"),
            (CssLength::Px(finite(0.0)), "0px"),
            (CssLength::Em(finite(1.5)), "1.5em"),
            (CssLength::Rem(finite(2.0)), "2rem"),
            (CssLength::Vw(finite(100.0)), "100vw"),
            (CssLength::Vh(finite(50.0)), "50vh"),
            (CssLength::Vmin(finite(25.0)), "25vmin"),
            (CssLength::Vmax(finite(75.0)), "75vmax"),
            (CssLength::Ch(finite(3.0)), "3ch"),
            (CssLength::Dvw(finite(50.0)), "50dvw"),
            (CssLength::Dvh(finite(100.0)), "100dvh"),
            (CssLength::Svw(finite(80.0)), "80svw"),
            (CssLength::Svh(finite(90.0)), "90svh"),
            (CssLength::Lvw(finite(100.0)), "100lvw"),
            (CssLength::Lvh(finite(100.0)), "100lvh"),
            (CssLength::Cqw(finite(50.0)), "50cqw"),
            (CssLength::Cqh(finite(25.0)), "25cqh"),
        ];
        for (length, expected) in cases {
            let mut buf = String::new();
            length.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_angle_write_to() {
        let cases = [
            (CssAngle::Deg(finite(45.0)), "45deg"),
            (CssAngle::Rad(finite(2.5)), "2.5rad"),
            (CssAngle::Turn(finite(0.5)), "0.5turn"),
            (CssAngle::Grad(finite(200.0)), "200grad"),
        ];
        for (angle, expected) in cases {
            let mut buf = String::new();
            angle.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_time_write_to() {
        let cases = [
            (CssTime::S(finite(0.3)), "0.3s"),
            (CssTime::Ms(finite(300.0)), "300ms"),
        ];
        for (time, expected) in cases {
            let mut buf = String::new();
            time.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_value_write_to() {
        let cases: Vec<(CssValue, &str)> = vec![
            (CssValue::Number(finite(0.5)), "0.5"),
            (CssValue::Integer(10), "10"),
            (CssValue::Length(CssLength::Px(finite(100.0))), "100px"),
            (CssValue::Percent(finite(50.0)), "50%"),
            (CssValue::Angle(CssAngle::Deg(finite(90.0))), "90deg"),
            (CssValue::Time(CssTime::S(finite(0.3))), "0.3s"),
            (CssValue::Fr(NonNegativeFiniteF64::new(1.0)), "1fr"),
            (CssValue::Auto, "auto"),
            (CssValue::Zero, "0px"),
            (CssValue::Inherit, "inherit"),
            (CssValue::Initial, "initial"),
            (CssValue::Unset, "unset"),
            (CssValue::Revert, "revert"),
            (CssValue::RevertLayer, "revert-layer"),
        ];
        for (value, expected) in cases {
            let mut buf = String::new();
            value.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_value_display() {
        assertr::assert_that!(px(10.0).to_string()).is_equal_to("10px".to_string());
        assertr::assert_that!(em(1.5).to_string()).is_equal_to("1.5em".to_string());
        assertr::assert_that!(pct(50.0).to_string()).is_equal_to("50%".to_string());
        assertr::assert_that!(deg(45.0).to_string()).is_equal_to("45deg".to_string());
        assertr::assert_that!(CssValue::Auto.to_string()).is_equal_to("auto".to_string());
        assertr::assert_that!(CssValue::Zero.to_string()).is_equal_to("0px".to_string());
    }

    #[test]
    fn test_convenience_functions() {
        assertr::assert_that!(format!("{}", px(100))).is_equal_to("100px".to_string());
        assertr::assert_that!(format!("{}", em(0.6))).is_equal_to("0.6em".to_string());
        assertr::assert_that!(format!("{}", rem(1.5))).is_equal_to("1.5rem".to_string());
        assertr::assert_that!(format!("{}", vw(100))).is_equal_to("100vw".to_string());
        assertr::assert_that!(format!("{}", vh(50))).is_equal_to("50vh".to_string());
        assertr::assert_that!(format!("{}", pct(75))).is_equal_to("75%".to_string());
        assertr::assert_that!(format!("{}", deg(180))).is_equal_to("180deg".to_string());
        assertr::assert_that!(format!("{}", s(0.3))).is_equal_to("0.3s".to_string());
        assertr::assert_that!(format!("{}", ms(300))).is_equal_to("300ms".to_string());
        assertr::assert_that!(format!("{}", fr(1))).is_equal_to("1fr".to_string());
    }

    #[test]
    fn test_modern_viewport_convenience_functions() {
        assertr::assert_that!(format!("{}", dvw(50))).is_equal_to("50dvw".to_string());
        assertr::assert_that!(format!("{}", dvh(100))).is_equal_to("100dvh".to_string());
        assertr::assert_that!(format!("{}", svw(80))).is_equal_to("80svw".to_string());
        assertr::assert_that!(format!("{}", svh(90))).is_equal_to("90svh".to_string());
        assertr::assert_that!(format!("{}", lvw(100))).is_equal_to("100lvw".to_string());
        assertr::assert_that!(format!("{}", lvh(100))).is_equal_to("100lvh".to_string());
        assertr::assert_that!(format!("{}", cqw(50))).is_equal_to("50cqw".to_string());
        assertr::assert_that!(format!("{}", cqh(25))).is_equal_to("25cqh".to_string());
    }

    #[test]
    fn test_css_dimension_modern_const_fns() {
        assertr::assert_that!(CssDimension::dvw(50.0).to_string()).is_equal_to("50dvw".to_string());
        assertr::assert_that!(CssDimension::dvh(100.0).to_string())
            .is_equal_to("100dvh".to_string());
        assertr::assert_that!(CssDimension::svw(80.0).to_string()).is_equal_to("80svw".to_string());
        assertr::assert_that!(CssDimension::svh(90.0).to_string()).is_equal_to("90svh".to_string());
        assertr::assert_that!(CssDimension::lvw(100.0).to_string())
            .is_equal_to("100lvw".to_string());
        assertr::assert_that!(CssDimension::lvh(100.0).to_string())
            .is_equal_to("100lvh".to_string());
        assertr::assert_that!(CssDimension::cqw(50.0).to_string()).is_equal_to("50cqw".to_string());
        assertr::assert_that!(CssDimension::cqh(25.0).to_string()).is_equal_to("25cqh".to_string());
    }

    #[test]
    fn test_integer_accepts_i32() {
        let v = CssValue::Integer(42);
        assertr::assert_that!(format!("{v}")).is_equal_to("42".to_string());
    }

    #[test]
    fn test_px_accepts_i32() {
        let v = px(10_i32);
        assertr::assert_that!(format!("{v}")).is_equal_to("10px".to_string());
    }

    #[test]
    fn test_css_color_write_to() {
        let mut buf = String::new();
        CssColor::Rgb(255, 128, 0).write_to(&mut buf);
        assertr::assert_that!(buf).is_equal_to("rgb(255, 128, 0)".to_string());

        let mut buf = String::new();
        CssColor::Rgba(0, 0, 0, UnitInterval::new(0.5)).write_to(&mut buf);
        assertr::assert_that!(buf).is_equal_to("rgba(0, 0, 0, 0.5)".to_string());
    }

    #[test]
    fn test_rgb_convenience() {
        assertr::assert_that!(rgb(255, 0, 0).to_string()).is_equal_to("rgb(255, 0, 0)".to_string());
        assertr::assert_that!(rgba(0, 0, 0, 0.5).to_string())
            .is_equal_to("rgba(0, 0, 0, 0.5)".to_string());
    }

    #[test]
    fn test_hsl_convenience() {
        assertr::assert_that!(hsl(120, 100, 50).to_string())
            .is_equal_to("hsl(120, 100%, 50%)".to_string());
        assertr::assert_that!(hsla(240, 50, 75, 0.8).to_string())
            .is_equal_to("hsla(240, 50%, 75%, 0.8)".to_string());
    }

    #[test]
    fn test_named_color() {
        let c = CssColor::Named(CssColorName::Transparent);
        assertr::assert_that!(c.to_string()).is_equal_to("transparent".to_string());
    }

    #[test]
    fn test_custom_property_names_are_checked() {
        let property = CssCustomProperty::<CssColor>::new("--accent-color");
        assertr::assert_that!(property.as_str()).is_equal_to("--accent-color");

        assert!(CssCustomProperty::<CssColor>::try_new("accent-color").is_err());
        assert!(CssCustomProperty::<CssColor>::try_new("--").is_err());
        assert!(CssCustomProperty::<CssColor>::try_new("--1accent").is_err());
        assert!(CssCustomProperty::<CssColor>::try_new("--accent;color").is_err());
    }

    #[test]
    fn test_typed_var_requires_and_serializes_typed_fallbacks() {
        let accent = CssCustomProperty::<CssColor>::new("--accent-color");
        let default_accent = CssCustomProperty::<CssColor>::new("--default-accent-color");
        let reference = var(
            &accent,
            var(&default_accent, CssColor::Named(CssColorName::CurrentColor)),
        );

        assertr::assert_that!(reference.to_string()).is_equal_to(
            "var(--accent-color, var(--default-accent-color, currentcolor))".to_string(),
        );
    }

    #[test]
    fn test_typed_var_participates_in_dimension_calculations() {
        let sidebar = CssCustomProperty::<CssDimension>::new("--sidebar-width");
        let reference = CssDimensionExpr::from(var(&sidebar, px(20)));
        let expression = CssDimensionExpr::from(pct(100)) - reference;
        let calculation = LengthPercentageCalculation::new(expression);

        assertr::assert_that!(calculation.to_string())
            .is_equal_to("calc(100% - var(--sidebar-width, 20px))".to_string());

        let direct_variable =
            LengthPercentageCalculation::new(CssDimensionExpr::from(var(&sidebar, px(-1))));
        assertr::assert_that!(direct_variable.to_string())
            .is_equal_to("calc(var(--sidebar-width, -1px))".to_string());
    }

    #[test]
    fn test_view_transition_names_are_checked() {
        assertr::assert_that!(ViewTransitionName::none().to_string())
            .is_equal_to("none".to_string());
        assertr::assert_that!(ViewTransitionName::named("hero-card").to_string())
            .is_equal_to("hero-card".to_string());

        for reserved in [
            "none",
            "auto",
            "default",
            "inherit",
            "initial",
            "revert",
            "revert-layer",
            "unset",
        ] {
            assert!(ViewTransitionName::try_named(reserved).is_err());
        }
        assert!(ViewTransitionName::try_named("hero card").is_err());
    }

    #[test]
    fn test_modern_color_adjust_values() {
        assertr::assert_that!(ForcedColorAdjust::PreserveParentColor.to_string())
            .is_equal_to("preserve-parent-color".to_string());
        assertr::assert_that!(PrintColorAdjust::Exact.to_string()).is_equal_to("exact".to_string());
    }

    #[test]
    fn test_from_i32() {
        let v: CssValue = 42_i32.into();
        assertr::assert_that!(v.to_string()).is_equal_to("42".to_string());
    }

    #[test]
    fn test_css_dimension_associated_fns() {
        assertr::assert_that!(CssDimension::em(1.5).to_string()).is_equal_to("1.5em".to_string());
        assertr::assert_that!(CssDimension::px(10.0).to_string()).is_equal_to("10px".to_string());
        assertr::assert_that!(CssDimension::pct(50.0).to_string()).is_equal_to("50%".to_string());
        assertr::assert_that!(CssDimension::rem(2.0).to_string()).is_equal_to("2rem".to_string());
    }

    #[test]
    fn test_css_dimension_const() {
        const DIM: CssDimension = CssDimension::em(3.5);
        assertr::assert_that!(DIM.to_string()).is_equal_to("3.5em".to_string());
    }

    #[test]
    fn test_css_dimension_is_copy() {
        let dim = CssDimension::px(10.0);
        let copy = dim;
        // Both are usable — dim was copied, not moved.
        assertr::assert_that!(dim.to_string()).is_equal_to(copy.to_string());
    }

    #[test]
    fn test_neg_css_length() {
        assertr::assert_that!((-CssLength::Px(finite(10.0))).to_string())
            .is_equal_to("-10px".to_string());
        assertr::assert_that!((-CssLength::Em(finite(1.5))).to_string())
            .is_equal_to("-1.5em".to_string());
    }

    #[test]
    fn test_neg_css_dimension() {
        assertr::assert_that!((-px(10)).to_string()).is_equal_to("-10px".to_string());
        assertr::assert_that!((-pct(50)).to_string()).is_equal_to("-50%".to_string());
        assertr::assert_that!((-CssDimension::Zero).to_string()).is_equal_to("0px".to_string());
    }

    #[test]
    fn test_neg_css_angle() {
        assertr::assert_that!((-CssAngle::Deg(finite(90.0))).to_string())
            .is_equal_to("-90deg".to_string());
    }

    #[test]
    fn test_neg_css_time() {
        assertr::assert_that!((-CssTime::Ms(finite(300.0))).to_string())
            .is_equal_to("-300ms".to_string());
    }

    #[test]
    fn test_global_keywords() {
        assertr::assert_that!(CssValue::Inherit.to_string()).is_equal_to("inherit".to_string());
        assertr::assert_that!(CssValue::Initial.to_string()).is_equal_to("initial".to_string());
        assertr::assert_that!(CssValue::Unset.to_string()).is_equal_to("unset".to_string());
        assertr::assert_that!(CssValue::Revert.to_string()).is_equal_to("revert".to_string());
        assertr::assert_that!(CssValue::RevertLayer.to_string())
            .is_equal_to("revert-layer".to_string());
    }

    #[test]
    fn test_non_negative_length_percentage_rejects_invalid_values() {
        assert!(matches!(
            NonNegativeLengthPercentage::try_from(px(-1)),
            Err(InvalidNonNegativeLengthPercentage::Negative(-1.0))
        ));
        assertr::assert_that!(NonNegativeLengthPercentage::new(pct(25)).to_string())
            .is_equal_to("25%".to_string());
    }

    #[test]
    fn test_padding_constructors_enforce_grammar() {
        assertr::assert_that!(Padding::all(px(8)).to_string()).is_equal_to("8px".to_string());
        assertr::assert_that!(Padding::double(px(8), pct(5)).to_string())
            .is_equal_to("8px 5%".to_string());
        assertr::assert_that!(Padding::calculated(pct(100) - px(20)).to_string())
            .is_equal_to("calc(100% - 20px)".to_string());
        assertr::assert_that!(PaddingAxis::start_end(px(8), pct(5)).to_string())
            .is_equal_to("8px 5%".to_string());
    }

    #[test]
    fn test_gap_uses_non_negative_components() {
        let row = GapValue::from(NonNegativeLengthPercentage::new(px(8)));
        let column = GapValue::from(NonNegativeLengthPercentage::new(pct(5)));
        assertr::assert_that!(Gap::RowColumn(row, column).to_string())
            .is_equal_to("8px 5%".to_string());

        let calculated = GapValue::from(LengthPercentageCalculation::new(pct(10) - px(2)));
        assertr::assert_that!(calculated.to_string()).is_equal_to("calc(10% - 2px)".to_string());
    }

    #[test]
    fn test_sizing_fit_content_and_logical_corner_radius() {
        assertr::assert_that!(Size::fit_content(px(320)).to_string())
            .is_equal_to("fit-content(320px)".to_string());
        assertr::assert_that!(MaxSize::fit_content_calculated(pct(100) - px(20)).to_string())
            .is_equal_to("fit-content(calc(100% - 20px))".to_string());
        assertr::assert_that!(BorderCornerRadius::elliptical(px(8), pct(50)).to_string())
            .is_equal_to("8px 50%".to_string());
    }

    #[test]
    fn test_finite_f64_try_new() {
        assertr::assert_that!(FiniteF64::try_new(12.5).expect("finite value").get())
            .is_equal_to(12.5);
        assert!(matches!(
            FiniteF64::try_new(f64::NAN),
            Err(InvalidCssNumber::NonFinite { .. })
        ));
        assert!(matches!(
            FiniteF64::try_new(f64::INFINITY),
            Err(InvalidCssNumber::NonFinite { .. })
        ));
    }

    #[test]
    fn test_fallible_numeric_constructors() {
        assertr::assert_that!(try_px(12.5).expect("finite px").to_string())
            .is_equal_to("12.5px".to_string());
        assertr::assert_that!(
            CssDimension::try_pct(25.0)
                .expect("finite percentage")
                .to_string()
        )
        .is_equal_to("25%".to_string());
        assertr::assert_that!(try_number(0.75).expect("finite number").to_string())
            .is_equal_to("0.75".to_string());

        assert!(matches!(
            try_px(f64::NAN),
            Err(InvalidCssNumber::NonFinite { context: "px", .. })
        ));
        assert!(matches!(
            try_pct(f64::INFINITY),
            Err(InvalidCssNumber::NonFinite { context: "%", .. })
        ));
        assert!(matches!(
            try_number(f64::NEG_INFINITY),
            Err(InvalidCssNumber::NonFinite {
                context: "number",
                ..
            })
        ));
    }

    #[test]
    fn test_try_from_f64_for_css_value() {
        let value = CssValue::try_from(1.25).expect("finite CSS number");
        assertr::assert_that!(value.to_string()).is_equal_to("1.25".to_string());
        assert!(CssValue::try_from(f64::NAN).is_err());
    }

    #[test]
    fn test_fallible_constrained_constructors() {
        assert!(matches!(
            try_fr(-1.0),
            Err(InvalidCssNumber::Negative {
                context: "non-negative number",
                ..
            })
        ));
        assert!(matches!(
            try_rgba(0, 0, 0, 1.5),
            Err(InvalidCssNumber::OutOfRange {
                context: "unit interval",
                ..
            })
        ));
        assert!(matches!(
            try_hsl(0.0, f64::NAN, 50.0),
            Err(InvalidCssNumber::NonFinite {
                context: "percentage channel",
                ..
            })
        ));
        assert!(matches!(
            try_hsla(0.0, 50.0, 50.0, 2.0),
            Err(InvalidCssNumber::OutOfRange {
                context: "unit interval",
                ..
            })
        ));
    }

    #[test]
    fn test_try_map_value_rejects_non_finite_result() {
        let length = CssLength::Px(finite(10.0));
        assert!(matches!(
            length.try_map_value(|_| f64::NAN),
            Err(InvalidCssNumber::NonFinite { .. })
        ));
    }

    #[test]
    #[should_panic(expected = "CSS number value must be finite")]
    fn test_map_value_non_finite_result_panics() {
        let _ = CssLength::Px(finite(10.0)).map_value(|_| f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "CSS number value must be finite")]
    fn test_same_unit_arithmetic_overflow_panics() {
        let _ = px(f64::MAX) + px(f64::MAX);
    }

    #[test]
    #[should_panic(expected = "CSS px value must be finite")]
    fn test_associated_px_nan_panics() {
        let _ = CssDimension::px(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS px value must be finite")]
    fn test_px_nan_panics() {
        let _ = px(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS % value must be finite")]
    fn test_pct_infinity_panics() {
        let _ = pct(f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "CSS deg value must be finite")]
    fn test_deg_nan_panics() {
        let _ = deg(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS rad value must be finite")]
    fn test_rad_infinity_panics() {
        let _ = rad(f64::NEG_INFINITY);
    }

    #[test]
    #[should_panic(expected = "CSS turn value must be finite")]
    fn test_turn_nan_panics() {
        let _ = turn(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS grad value must be finite")]
    fn test_grad_nan_panics() {
        let _ = grad(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS s value must be finite")]
    fn test_s_nan_panics() {
        let _ = s(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS ms value must be finite")]
    fn test_ms_infinity_panics() {
        let _ = ms(f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "CSS non-negative number must be non-negative")]
    fn test_fr_negative_panics() {
        let _ = fr(-1.0);
    }

    #[test]
    #[should_panic(expected = "CSS unit interval must be in range [0, 1]")]
    fn test_rgba_alpha_out_of_range_panics() {
        let _ = rgba(0, 0, 0, 1.5);
    }

    #[test]
    #[should_panic(expected = "CSS percentage channel must be in range [0, 100]")]
    fn test_hsl_saturation_out_of_range_panics() {
        let _ = hsl(120.0, 120.0, 50.0);
    }

    #[test]
    #[should_panic(expected = "CSS hsl hue value must be finite")]
    fn test_hsl_hue_nan_panics() {
        let _ = hsl(f64::NAN, 50.0, 50.0);
    }

    #[test]
    #[should_panic(expected = "CSS unit interval must be in range [0, 1]")]
    fn test_hsla_alpha_out_of_range_panics() {
        let _ = hsla(240.0, 50.0, 50.0, -0.1);
    }

    #[test]
    #[should_panic(expected = "CSS hsla hue value must be finite")]
    fn test_hsla_hue_infinity_panics() {
        let _ = hsla(f64::INFINITY, 50.0, 50.0, 1.0);
    }

    #[test]
    fn test_css_dimension_expr_simple() {
        let expr = CssDimensionExpr::from(px(100));
        assertr::assert_that!(expr.to_string()).is_equal_to("100px".to_string());
    }

    #[test]
    fn test_css_dimension_expr_calc() {
        assertr::assert_that!((pct(100) - px(20)).to_string())
            .is_equal_to("calc(100% - 20px)".to_string());
    }

    #[test]
    fn test_length_percentage_calculation_wraps_simple_out_of_range_value() {
        let calculation = LengthPercentageCalculation::new(px(-1));
        assertr::assert_that!(calculation.to_string()).is_equal_to("calc(-1px)".to_string());
    }

    #[test]
    fn test_length_percentage_calculation_does_not_double_wrap_function_expression() {
        let calculation = LengthPercentageCalculation::new(css_min(vw(50), px(300)));
        assertr::assert_that!(calculation.to_string()).is_equal_to("min(50vw, 300px)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_min() {
        assertr::assert_that!(css_min(vw(50), px(300)).to_string())
            .is_equal_to("min(50vw, 300px)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_max() {
        assertr::assert_that!(css_max(px(200), pct(50)).to_string())
            .is_equal_to("max(200px, 50%)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_clamp() {
        assertr::assert_that!(css_clamp(px(200), pct(50), px(800)).to_string())
            .is_equal_to("clamp(200px, 50%, 800px)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_env() {
        assertr::assert_that!(css_env(CssEnvironmentVariable::SafeAreaInsetTop).to_string())
            .is_equal_to("env(safe-area-inset-top)".to_string());
    }

    #[test]
    fn test_dimension_add_same_unit_folds() {
        let result = px(10) + px(5);
        assertr::assert_that!(result.to_string()).is_equal_to("15px".to_string());
    }

    #[test]
    fn test_dimension_sub_same_unit_folds() {
        let result = px(10) - px(3);
        assertr::assert_that!(result.to_string()).is_equal_to("7px".to_string());
    }

    #[test]
    fn test_dimension_add_percent_folds() {
        let result = pct(60) + pct(40);
        assertr::assert_that!(result.to_string()).is_equal_to("100%".to_string());
    }

    #[test]
    fn test_dimension_add_mixed_produces_calc() {
        let result = pct(100) - px(20);
        assertr::assert_that!(result.to_string()).is_equal_to("calc(100% - 20px)".to_string());
    }

    #[test]
    fn test_dimension_add_zero_simplifies() {
        let result = px(10) + CssDimension::Zero;
        assertr::assert_that!(result.to_string()).is_equal_to("10px".to_string());
    }

    #[test]
    fn test_dimension_sub_zero_simplifies() {
        let result = pct(50) - CssDimension::Zero;
        assertr::assert_that!(result.to_string()).is_equal_to("50%".to_string());
    }

    #[test]
    fn test_dimension_expr_add_dimension() {
        let expr = (pct(100) - px(20)) + px(10);
        assertr::assert_that!(expr.to_string())
            .is_equal_to("calc((100% - 20px) + 10px)".to_string());
    }

    #[test]
    fn test_dimension_expr_add_expr() {
        let a = CssDimensionExpr::from(px(10));
        let b = CssDimensionExpr::from(px(5));
        let result = a + b;
        assertr::assert_that!(result.to_string()).is_equal_to("15px".to_string());
    }

    #[test]
    fn test_margin_accepts_auto_through_its_own_grammar_type() {
        let margin = Margin::All(LengthPercentageAuto::Auto);
        assertr::assert_that!(margin.to_string()).is_equal_to("auto".to_string());
    }
}
