//! Direct Leptos integration tests for typed CSS property values.

use leptos::tachys::html::{attribute::Attribute, style::IntoStyle};
use leptos_css::{
    BorderCornerRadius, CssColor, CssColorName, CssCustomProperty, CssDimension, CssDimensionExpr,
    ForcedColorAdjust, GlobalKeyword, Inset, LengthPercentageAuto, LengthPercentageCalculation,
    MarginAxis, MaxSize, NonNegativeLengthPercentage, Padding, PaddingAxis, PrintColorAdjust,
    PropertyName, Size, TouchAction, TouchActionGestures, TouchActionHorizontalPan,
    TouchActionVerticalPan, ViewTransitionName, css_custom_property, pct,
    property::{
        AllProperty, BackgroundColorProperty, BorderStartStartRadiusProperty, ColorProperty,
        ForcedColorAdjustProperty, InsetBlockStartProperty, InsetProperty, MarginInlineProperty,
        MaxWidthProperty, MinWidthProperty, PaddingBlockProperty, PaddingProperty,
        PrintColorAdjustProperty, TouchActionProperty, ViewTransitionNameProperty, WidthProperty,
    },
    px, rgb, var,
};

css_custom_property!(ACCENT_COLOR: CssColor = "--accent-color");

fn render_style_attribute(attribute: impl Attribute) -> String {
    let mut html = String::new();
    let mut class = String::new();
    let mut styles = String::new();
    let mut inner_html = String::new();

    attribute.to_html(&mut html, &mut class, &mut styles, &mut inner_html);
    styles
}

fn render_style(style: impl IntoStyle) -> String {
    let mut output = String::new();
    style.to_html(&mut output);
    output
}

#[test]
fn all_property_accepts_only_a_global_keyword() {
    let attribute = AllProperty
        .declare(GlobalKeyword::RevertLayer)
        .into_attribute();

    assert_eq!(render_style_attribute(attribute), "all:revert-layer;");
}

#[test]
fn ordinary_property_marker_accepts_global_through_separate_method() {
    let attribute = PaddingProperty.declare_global(GlobalKeyword::Inherit);

    assert_eq!(render_style(attribute), "padding:inherit;");
}

#[test]
fn typed_property_marker_accepts_its_exact_grammar_during_ssr() {
    let gestures = TouchActionGestures::horizontal(TouchActionHorizontalPan::PanLeft)
        .with_vertical(TouchActionVerticalPan::PanUp)
        .with_pinch_zoom();
    let attribute = TouchActionProperty.declare(TouchAction::Gestures(gestures));

    assert_eq!(
        render_style(attribute),
        "touch-action:pan-left pan-up pinch-zoom;"
    );
}

#[test]
fn property_markers_render_checked_values_and_typed_expressions() {
    let padding = render_style(PaddingProperty.declare(Padding::all(px(16))));
    let width = render_style(WidthProperty.declare(Size::Calculation(
        LengthPercentageCalculation::new(pct(100) - px(20)),
    )));
    let color = render_style(ColorProperty.declare(CssColor::Named(CssColorName::Red)));

    assert_eq!(padding, "padding:16px;");
    assert_eq!(width, "width:calc(100% - 20px);");
    assert_eq!(color, "color:red;");
}

#[test]
fn sizing_level_four_keywords_render_for_preferred_minimum_and_maximum_properties() {
    let preferred = render_style(WidthProperty.declare(Size::FitContent));
    let minimum = render_style(MinWidthProperty.declare(Size::Contain));
    let maximum = render_style(MaxWidthProperty.declare(MaxSize::Stretch));

    assert_eq!(preferred, "width:fit-content;");
    assert_eq!(minimum, "min-width:contain;");
    assert_eq!(maximum, "max-width:stretch;");
}

#[test]
fn one_value_grammar_can_build_distinct_checked_properties() {
    let red = CssColor::Named(CssColorName::Red);
    let foreground = ColorProperty.declare(red);
    let background = BackgroundColorProperty.declare(red);

    assert_eq!(ColorProperty.property_name(), PropertyName::Color.as_str());
    assert_eq!(
        BackgroundColorProperty.property_name(),
        PropertyName::BackgroundColor.as_str()
    );
    assert_eq!(foreground.property_name(), "color");
    assert_eq!(background.property_name(), "background-color");
    assert_ne!(foreground, background);

    let mut output = String::new();
    foreground.clone().write_declaration_to(&mut output);
    assert_eq!(output, "color:red;");
    assert_eq!(foreground, foreground.clone());
}

#[test]
fn color_helpers_feed_every_property_using_the_color_grammar() {
    let foreground = ColorProperty.declare(rgb(12, 34, 56));
    let background = BackgroundColorProperty.declare(rgb(12, 34, 56));

    assert_eq!(render_style(foreground), "color:rgb(12, 34, 56);");
    assert_eq!(
        render_style(background),
        "background-color:rgb(12, 34, 56);"
    );
}

#[test]
fn longhand_marker_accepts_only_non_negative_length_percentage() {
    let value = NonNegativeLengthPercentage::new(px(12));
    let attribute = leptos_css::property::PaddingTopProperty.declare(value);

    assert_eq!(render_style(attribute), "padding-top:12px;");
}

#[test]
fn typed_custom_properties_and_var_render_during_ssr() {
    let declaration = render_style(ACCENT_COLOR.declare(CssColor::Named(CssColorName::Aqua)));
    let runtime_accent = CssCustomProperty::<CssColor>::new("--runtime-accent-color");
    let runtime_declaration =
        render_style(runtime_accent.declare(CssColor::Named(CssColorName::Red)));
    let reference = render_style(ColorProperty.declare(var(
        &ACCENT_COLOR,
        CssColor::Named(CssColorName::CurrentColor),
    )));

    assert_eq!(declaration, "--accent-color:aqua;");
    assert_eq!(runtime_declaration, "--runtime-accent-color:red;");
    assert_eq!(reference, "color:var(--accent-color, currentcolor);");
}

#[test]
fn sizing_and_touch_action_grammars_can_type_custom_properties() {
    let preferred_size = CssCustomProperty::<Size>::new("--preferred-size");
    let maximum_size = CssCustomProperty::<MaxSize>::new("--maximum-size");
    let horizontal_pan = CssCustomProperty::<TouchActionHorizontalPan>::new("--horizontal-pan");
    let vertical_pan = CssCustomProperty::<TouchActionVerticalPan>::new("--vertical-pan");
    let gestures = CssCustomProperty::<TouchActionGestures>::new("--gestures");
    let touch_action = CssCustomProperty::<TouchAction>::new("--touch-action");

    assert_eq!(
        render_style(preferred_size.declare(Size::FitContent)),
        "--preferred-size:fit-content;"
    );
    assert_eq!(
        render_style(maximum_size.declare(MaxSize::Contain)),
        "--maximum-size:contain;"
    );
    assert_eq!(
        render_style(horizontal_pan.declare(TouchActionHorizontalPan::PanLeft)),
        "--horizontal-pan:pan-left;"
    );
    assert_eq!(
        render_style(vertical_pan.declare(TouchActionVerticalPan::PanUp)),
        "--vertical-pan:pan-up;"
    );
    assert_eq!(
        render_style(
            gestures.declare(
                TouchActionGestures::horizontal(TouchActionHorizontalPan::PanX)
                    .with_vertical(TouchActionVerticalPan::PanY)
                    .with_pinch_zoom()
            )
        ),
        "--gestures:pan-x pan-y pinch-zoom;"
    );
    assert_eq!(
        render_style(touch_action.declare(TouchAction::Manipulation)),
        "--touch-action:manipulation;"
    );
}

#[test]
fn typed_variables_work_inside_checked_size_calculations() {
    let sidebar = CssCustomProperty::<CssDimension>::new("--sidebar-width");
    let expression =
        CssDimensionExpr::from(pct(100)) - CssDimensionExpr::from(var(&sidebar, px(320)));
    let width = render_style(WidthProperty.declare(Size::Calculation(
        LengthPercentageCalculation::new(expression),
    )));

    assert_eq!(width, "width:calc(100% - var(--sidebar-width, 320px));");
}

#[test]
fn modern_property_markers_render_their_closed_grammars() {
    let forced =
        render_style(ForcedColorAdjustProperty.declare(ForcedColorAdjust::PreserveParentColor));
    let print = render_style(PrintColorAdjustProperty.declare(PrintColorAdjust::Exact));
    let transition =
        render_style(ViewTransitionNameProperty.declare(ViewTransitionName::named("hero-card")));
    let radius = render_style(
        BorderStartStartRadiusProperty.declare(BorderCornerRadius::elliptical(px(8), pct(50))),
    );

    assert_eq!(forced, "forced-color-adjust:preserve-parent-color;");
    assert_eq!(print, "print-color-adjust:exact;");
    assert_eq!(transition, "view-transition-name:hero-card;");
    assert_eq!(radius, "border-start-start-radius:8px 50%;");
}

#[test]
fn logical_box_model_markers_render_checked_shorthands_and_longhands() {
    let padding = render_style(PaddingBlockProperty.declare(PaddingAxis::start_end(px(8), pct(5))));
    let margin = render_style(MarginInlineProperty.declare(MarginAxis::StartEnd(
        LengthPercentageAuto::Auto,
        LengthPercentageAuto::from(px(-8)),
    )));
    let inset = render_style(
        InsetProperty.declare(Inset::All(LengthPercentageAuto::from(pct(50) - px(20)))),
    );
    let inset_start = render_style(InsetBlockStartProperty.declare(LengthPercentageAuto::Auto));

    assert_eq!(padding, "padding-block:8px 5%;");
    assert_eq!(margin, "margin-inline:auto -8px;");
    assert_eq!(inset, "inset:calc(50% - 20px);");
    assert_eq!(inset_start, "inset-block-start:auto;");
}
