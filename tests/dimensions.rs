//! Public-surface tests for dimensions.

use assertr::prelude::*;
use leptos_css::*;

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
    assertr::assert_that!(CssDimension::dvh(100.0).to_string()).is_equal_to("100dvh".to_string());
    assertr::assert_that!(CssDimension::svw(80.0).to_string()).is_equal_to("80svw".to_string());
    assertr::assert_that!(CssDimension::svh(90.0).to_string()).is_equal_to("90svh".to_string());
    assertr::assert_that!(CssDimension::lvw(100.0).to_string()).is_equal_to("100lvw".to_string());
    assertr::assert_that!(CssDimension::lvh(100.0).to_string()).is_equal_to("100lvh".to_string());
    assertr::assert_that!(CssDimension::cqw(50.0).to_string()).is_equal_to("50cqw".to_string());
    assertr::assert_that!(CssDimension::cqh(25.0).to_string()).is_equal_to("25cqh".to_string());
}

#[test]
fn test_px_accepts_i32() {
    let v = px(10_i32);
    assertr::assert_that!(format!("{v}")).is_equal_to("10px".to_string());
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
    assertr::assert_that!(expr.to_string()).is_equal_to("calc((100% - 20px) + 10px)".to_string());
}

#[test]
fn test_dimension_expr_add_expr() {
    let a = CssDimensionExpr::from(px(10));
    let b = CssDimensionExpr::from(px(5));
    let result = a + b;
    assertr::assert_that!(result.to_string()).is_equal_to("15px".to_string());
}
