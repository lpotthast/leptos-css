//! Public-surface tests for colors.

use assertr::prelude::*;
use leptos_css::*;

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
fn test_modern_color_adjust_values() {
    assertr::assert_that!(ForcedColorAdjust::PreserveParentColor.to_string())
        .is_equal_to("preserve-parent-color".to_string());
    assertr::assert_that!(PrintColorAdjust::Exact.to_string()).is_equal_to("exact".to_string());
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
