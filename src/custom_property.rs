use std::{borrow::Cow, fmt, marker::PhantomData};

use crate::{
    CheckedCssValue, CheckedDeclaration, checked_value,
    identifier::is_checked_custom_property_name, serialization::CssWriteTo,
};

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

/// A validated CSS custom property associated with one Rust value grammar.
///
/// The type parameter prevents ordinary checked code from using, for example, a color variable
/// as a size. The same CSS name must not be independently constructed with two different Rust
/// grammar types; conflicting handwritten or external declarations are outside this crate's
/// checked boundary. Constructors are available only when `T` implements the sealed
/// [`CheckedCssValue`] marker.
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

impl<T: CheckedCssValue> CssCustomProperty<T> {
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

    /// Assign a checked value to this custom property.
    ///
    /// The value may be a literal of `T` or a typed [`crate::var`] reference with the same
    /// grammar. Arbitrary custom-property token streams are intentionally unsupported.
    pub fn declare(&self, value: impl Into<DeclarationValue<T>>) -> CheckedDeclaration {
        CheckedDeclaration::new(self.name.clone(), value.into())
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

impl<T: CheckedCssValue> fmt::Display for CssCustomProperty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A checked literal or a typed custom-property reference for one CSS grammar.
///
/// The representation is private so callers cannot construct an untyped variable reference or
/// raw fallback token stream. Public conversions are available only for [`CheckedCssValue`]
/// grammars.
#[derive(Clone, Debug, PartialEq)]
pub struct DeclarationValue<T>(DeclarationValueKind<T>);

#[derive(Clone, Debug, PartialEq)]
enum DeclarationValueKind<T> {
    Literal(T),
    Variable(CssVariableReference<T>),
}

impl<T: CheckedCssValue> From<T> for DeclarationValue<T> {
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

impl<T: CheckedCssValue> From<CssVariableReference<T>> for DeclarationValue<T> {
    fn from(value: CssVariableReference<T>) -> Self {
        Self(DeclarationValueKind::Variable(value))
    }
}

impl<T: CheckedCssValue> checked_value::sealed::Sealed for DeclarationValue<T> {}

impl<T: CheckedCssValue> checked_value::sealed::Sealed for CssVariableReference<T> {}

impl<T: CheckedCssValue> CssWriteTo for DeclarationValue<T> {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match &self.0 {
            DeclarationValueKind::Literal(value) => value.css_fmt(w),
            DeclarationValueKind::Variable(value) => value.css_fmt(w),
        }
    }
}

impl<T: CheckedCssValue> CssWriteTo for CssVariableReference<T> {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "var({}, ", self.property)?;
        self.fallback.css_fmt(w)?;
        w.write_char(')')
    }
}

impl<T: CheckedCssValue> fmt::Display for DeclarationValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.css_fmt(f)
    }
}

impl<T: CheckedCssValue> fmt::Display for CssVariableReference<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.css_fmt(f)
    }
}

/// Reference a typed custom property using a mandatory fallback of the same grammar.
///
/// Requiring a fallback prevents an undefined custom property from invalidating the consuming
/// declaration at computed-value time. The fallback may itself be another typed `var()` result.
/// Both the property grammar and fallback must implement the sealed [`CheckedCssValue`] marker;
/// [`CssWriteTo`] alone is only a serialization capability.
///
/// ```compile_fail
/// use leptos_css::{CssCustomProperty, CssWriteTo, var};
///
/// fn reference<T: CssWriteTo>(property: &CssCustomProperty<T>, fallback: T) {
///     let _ = var(property, fallback);
/// }
/// ```
///
/// [CSS variable substitution]: https://www.w3.org/TR/css-variables-1/#using-variables
pub fn var<T: CheckedCssValue>(
    property: &CssCustomProperty<T>,
    fallback: impl Into<DeclarationValue<T>>,
) -> CssVariableReference<T> {
    CssVariableReference {
        property: property.clone(),
        fallback: Box::new(fallback.into()),
    }
}
