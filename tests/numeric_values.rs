//! Public-surface tests for numeric values.

use assertr::prelude::*;
use leptos_css::*;

fn finite(value: f64) -> FiniteF64 {
    FiniteF64::new(value)
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
fn test_integer_accepts_i32() {
    let v = CssValue::Integer(42);
    assertr::assert_that!(format!("{v}")).is_equal_to("42".to_string());
}

#[test]
fn test_from_i32() {
    let v: CssValue = 42_i32.into();
    assertr::assert_that!(v.to_string()).is_equal_to("42".to_string());
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
fn test_finite_f64_try_new() {
    assertr::assert_that!(FiniteF64::try_new(12.5).expect("finite value").get()).is_equal_to(12.5);
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
