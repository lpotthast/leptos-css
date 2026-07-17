//! Public-surface tests for custom properties.

use assertr::prelude::*;
use leptos_css::*;

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

    assertr::assert_that!(reference.to_string())
        .is_equal_to("var(--accent-color, var(--default-accent-color, currentcolor))".to_string());
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
