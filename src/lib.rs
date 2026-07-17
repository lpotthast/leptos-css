#![doc = include_str!("../README.md")]

mod declaration;
/// CSS property-name catalog and grammar-checked Leptos property markers.
pub mod property;
/// Typed CSS value primitives and closed expression trees.
pub mod value;

pub use declaration::CheckedDeclaration;
pub use property::PropertyName;
pub use value::*;

/// Compile-time assertions that the typed CSS surface is `Send + Sync`. Both
/// `CssValue`, `PropertyName`, and checked declarations flow through Leptos component props and reactive
/// signals, so weakening either bound would surface at every downstream
/// consumer. Catch the regression at the crate boundary instead.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CssValue>();
    assert_send_sync::<CssDimensionExpr>();
    assert_send_sync::<CssCustomProperty<CssColor>>();
    assert_send_sync::<DeclarationValue<CssColor>>();
    assert_send_sync::<Padding>();
    assert_send_sync::<Inset>();
    assert_send_sync::<ViewTransitionName>();
    assert_send_sync::<TouchAction>();
    assert_send_sync::<PropertyName>();
    assert_send_sync::<CheckedDeclaration>();
};
