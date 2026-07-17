//! Public-surface tests for property grammars.

use assertr::prelude::*;
use leptos_css::*;

#[test]
fn test_view_transition_names_are_checked() {
    assertr::assert_that!(ViewTransitionName::none().to_string()).is_equal_to("none".to_string());
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
fn test_touch_action_gesture_composition_covers_normative_grammar() {
    assertr::assert_that!(TouchAction::Auto.to_string()).is_equal_to("auto".to_string());
    assertr::assert_that!(TouchAction::None.to_string()).is_equal_to("none".to_string());
    assertr::assert_that!(TouchAction::Manipulation.to_string())
        .is_equal_to("manipulation".to_string());

    let horizontal = [
        (TouchActionHorizontalPan::PanX, "pan-x"),
        (TouchActionHorizontalPan::PanLeft, "pan-left"),
        (TouchActionHorizontalPan::PanRight, "pan-right"),
    ];
    let vertical = [
        (TouchActionVerticalPan::PanY, "pan-y"),
        (TouchActionVerticalPan::PanUp, "pan-up"),
        (TouchActionVerticalPan::PanDown, "pan-down"),
    ];

    for (horizontal, expected_horizontal) in horizontal {
        let gestures = TouchActionGestures::horizontal(horizontal);
        assertr::assert_that!(gestures.to_string()).is_equal_to(expected_horizontal.to_string());
        assertr::assert_that!(gestures.with_pinch_zoom().to_string())
            .is_equal_to(format!("{expected_horizontal} pinch-zoom"));

        for (vertical, expected_vertical) in vertical {
            let gestures = gestures.with_vertical(vertical);
            assertr::assert_that!(gestures.to_string())
                .is_equal_to(format!("{expected_horizontal} {expected_vertical}"));
            assertr::assert_that!(gestures.with_pinch_zoom().to_string()).is_equal_to(format!(
                "{expected_horizontal} {expected_vertical} pinch-zoom"
            ));
        }
    }

    for (vertical, expected_vertical) in vertical {
        let gestures = TouchActionGestures::vertical(vertical);
        assertr::assert_that!(gestures.to_string()).is_equal_to(expected_vertical.to_string());
        assertr::assert_that!(gestures.with_pinch_zoom().to_string())
            .is_equal_to(format!("{expected_vertical} pinch-zoom"));
    }

    assertr::assert_that!(TouchActionGestures::pinch_zoom().to_string())
        .is_equal_to("pinch-zoom".to_string());
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
fn test_sizing_level_four_keywords_and_fit_content_functions() {
    assertr::assert_that!(Size::Stretch.to_string()).is_equal_to("stretch".to_string());
    assertr::assert_that!(Size::FitContent.to_string()).is_equal_to("fit-content".to_string());
    assertr::assert_that!(Size::Contain.to_string()).is_equal_to("contain".to_string());
    assertr::assert_that!(Size::fit_content(px(320)).to_string())
        .is_equal_to("fit-content(320px)".to_string());

    assertr::assert_that!(MaxSize::Stretch.to_string()).is_equal_to("stretch".to_string());
    assertr::assert_that!(MaxSize::FitContent.to_string()).is_equal_to("fit-content".to_string());
    assertr::assert_that!(MaxSize::Contain.to_string()).is_equal_to("contain".to_string());
    assertr::assert_that!(MaxSize::fit_content_calculated(pct(100) - px(20)).to_string())
        .is_equal_to("fit-content(calc(100% - 20px))".to_string());
}

#[test]
fn test_logical_corner_radius() {
    assertr::assert_that!(BorderCornerRadius::elliptical(px(8), pct(50)).to_string())
        .is_equal_to("8px 50%".to_string());
}

#[test]
fn test_margin_accepts_auto_through_its_own_grammar_type() {
    let margin = Margin::All(LengthPercentageAuto::Auto);
    assertr::assert_that!(margin.to_string()).is_equal_to("auto".to_string());
}
