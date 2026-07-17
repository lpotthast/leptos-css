use std::borrow::Cow;

use leptos::tachys::{
    html::{
        attribute::Attribute,
        style::{IntoStyle, style},
    },
    renderer::{Rndr, dom::CssStyleDeclaration, types::Element},
};

use crate::CheckedCssValue;

/// An owned CSS declaration whose property and value grammar were checked together.
///
/// The representation is private so a property name cannot be separated from its checked value
/// and recombined with another value. Construction is restricted to crate-owned boundaries that
/// pair a validated name with a sealed [`crate::CheckedCssValue`] grammar.
///
/// Different checked grammars and properties erase into the same storable type:
///
/// ```rust
/// use leptos_css::{
///     CheckedDeclaration, CssColor, Padding, css_custom_property, px, rgb,
///     property::{ColorProperty, PaddingProperty},
/// };
///
/// css_custom_property!(ACCENT: CssColor = "--accent");
/// let declarations: Vec<CheckedDeclaration> = vec![
///     PaddingProperty.declare(Padding::all(px(16))),
///     ColorProperty.declare(rgb(255, 0, 0)),
///     ACCENT.declare(rgb(0, 0, 255)),
/// ];
/// assert_eq!(declarations[0].property_name(), "padding");
/// ```
///
/// Its private constructor prevents unchecked property/value recombination:
///
/// ```compile_fail
/// use std::borrow::Cow;
/// use leptos_css::{CheckedDeclaration, rgb};
///
/// let _ = CheckedDeclaration::new(Cow::Borrowed("padding"), rgb(255, 0, 0));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckedDeclaration {
    property: Cow<'static, str>,
    value: String,
}

impl CheckedDeclaration {
    pub(crate) fn new(property: Cow<'static, str>, value: impl CheckedCssValue) -> Self {
        // Most primitive and shorthand values fit without growing this initial allocation.
        let mut rendered = String::with_capacity(32);
        value.write_to(&mut rendered);
        Self {
            property,
            value: rendered,
        }
    }

    /// Return the checked CSS property name.
    #[must_use]
    pub fn property_name(&self) -> &str {
        &self.property
    }

    /// Append the complete `property:value;` declaration to a string buffer.
    pub fn write_declaration_to(&self, output: &mut String) {
        output.push_str(&self.property);
        output.push(':');
        output.push_str(&self.value);
        output.push(';');
    }

    /// Convert this declaration into a spreadable Leptos style attribute.
    pub fn into_attribute(self) -> impl Attribute {
        style(self)
    }
}

fn set_declaration(style: &CssStyleDeclaration, declaration: &CheckedDeclaration) {
    Rndr::set_css_property(style, declaration.property_name(), &declaration.value);
}

impl IntoStyle for CheckedDeclaration {
    type AsyncOutput = Self;
    type State = (CssStyleDeclaration, Self);
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn to_html(self, style: &mut String) {
        self.write_declaration_to(style);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        (Rndr::style(el), self)
    }

    fn build(self, el: &Element) -> Self::State {
        let style = Rndr::style(el);
        set_declaration(&style, &self);
        (style, self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (style, current) = state;
        if self.property != current.property {
            Rndr::remove_css_property(style, current.property_name());
            set_declaration(style, &self);
        } else if self.value != current.value {
            set_declaration(style, &self);
        }
        *current = self;
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        Rndr::remove_css_property(&state.0, state.1.property_name());
    }
}
