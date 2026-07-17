use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::{identifier::is_checked_identifier, serialization::CssWriteTo};

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

impl_checked_value!(ViewTransitionName);
