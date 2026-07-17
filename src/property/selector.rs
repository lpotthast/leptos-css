//! Grammar-checked selectors for the supported declaration surface.

use std::borrow::Cow;

use crate::{
    BorderCornerRadius, CheckedDeclaration, CssColor, DeclarationValue, FontWeight,
    ForcedColorAdjust, Gap, GapValue, GlobalKeyword, Inset, InsetAxis, LengthPercentageAuto,
    Margin, MarginAxis, MaxSize, NonNegativeLengthPercentageValue, Opacity, Padding, PaddingAxis,
    PrintColorAdjust, Size, TouchAction, ViewTransitionName, ZIndex,
};

use super::PropertyName;

macro_rules! typed_property {
    ($name:ident, $variant:ident, $value:ty, $spec:literal) => {
        #[doc = concat!("Checked selector for [`PropertyName::", stringify!($variant), "`].")]
        #[doc = concat!("

Accepts the [`", stringify!($value), "`] grammar.")]
        #[doc = concat!("

[Normative specification](", $spec, ").")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $name {
            /// Return the selected CSS property name.
            #[must_use]
            pub const fn property_name(&self) -> &'static str {
                PropertyName::$variant.as_str()
            }

            /// Construct a grammar-checked declaration for this property.
            pub fn declare(
                &self,
                value: impl Into<DeclarationValue<$value>>,
            ) -> CheckedDeclaration {
                CheckedDeclaration::new(Cow::Borrowed(self.property_name()), value.into())
            }

            /// Construct a checked declaration using a CSS-wide keyword.
            pub fn declare_global(&self, value: GlobalKeyword) -> CheckedDeclaration {
                CheckedDeclaration::new(Cow::Borrowed(self.property_name()), value)
            }
        }
    };
}

/// Checked selector for the [`all` property](https://www.w3.org/TR/css-cascade-6/#all-shorthand).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllProperty;

impl AllProperty {
    /// Return the selected CSS property name.
    #[must_use]
    pub const fn property_name(&self) -> &'static str {
        PropertyName::All.as_str()
    }

    /// Construct a grammar-checked `all` declaration.
    pub fn declare(&self, value: GlobalKeyword) -> CheckedDeclaration {
        CheckedDeclaration::new(Cow::Borrowed(self.property_name()), value)
    }
}

typed_property!(
    TouchActionProperty,
    TouchAction,
    TouchAction,
    "https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property"
);
typed_property!(
    PaddingProperty,
    Padding,
    Padding,
    "https://www.w3.org/TR/css-box-4/#padding-shorthand"
);
typed_property!(
    PaddingTopProperty,
    PaddingTop,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingRightProperty,
    PaddingRight,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingBottomProperty,
    PaddingBottom,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingLeftProperty,
    PaddingLeft,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingBlockStartProperty,
    PaddingBlockStart,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingBlockEndProperty,
    PaddingBlockEnd,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingInlineStartProperty,
    PaddingInlineStart,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingInlineEndProperty,
    PaddingInlineEnd,
    NonNegativeLengthPercentageValue,
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingBlockProperty,
    PaddingBlock,
    PaddingAxis,
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingInlineProperty,
    PaddingInline,
    PaddingAxis,
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    MarginProperty,
    Margin,
    Margin,
    "https://www.w3.org/TR/css-box-4/#margin-shorthand"
);
typed_property!(
    MarginTopProperty,
    MarginTop,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginRightProperty,
    MarginRight,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginBottomProperty,
    MarginBottom,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginLeftProperty,
    MarginLeft,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginBlockStartProperty,
    MarginBlockStart,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginBlockEndProperty,
    MarginBlockEnd,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginInlineStartProperty,
    MarginInlineStart,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginInlineEndProperty,
    MarginInlineEnd,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginBlockProperty,
    MarginBlock,
    MarginAxis,
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginInlineProperty,
    MarginInline,
    MarginAxis,
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    GapProperty,
    Gap,
    Gap,
    "https://www.w3.org/TR/css-align-3/#gap-shorthand"
);
typed_property!(
    RowGapProperty,
    RowGap,
    GapValue,
    "https://www.w3.org/TR/css-align-3/#column-row-gap"
);
typed_property!(
    ColumnGapProperty,
    ColumnGap,
    GapValue,
    "https://www.w3.org/TR/css-align-3/#column-row-gap"
);
typed_property!(
    WidthProperty,
    Width,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    HeightProperty,
    Height,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    BlockSizeProperty,
    BlockSize,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    InlineSizeProperty,
    InlineSize,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    MinWidthProperty,
    MinWidth,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MinHeightProperty,
    MinHeight,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MinBlockSizeProperty,
    MinBlockSize,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MinInlineSizeProperty,
    MinInlineSize,
    Size,
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MaxWidthProperty,
    MaxWidth,
    MaxSize,
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    MaxHeightProperty,
    MaxHeight,
    MaxSize,
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    MaxBlockSizeProperty,
    MaxBlockSize,
    MaxSize,
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    MaxInlineSizeProperty,
    MaxInlineSize,
    MaxSize,
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    TopProperty,
    Top,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    RightProperty,
    Right,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    BottomProperty,
    Bottom,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    LeftProperty,
    Left,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    InsetProperty,
    Inset,
    Inset,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetBlockProperty,
    InsetBlock,
    InsetAxis,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetInlineProperty,
    InsetInline,
    InsetAxis,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetBlockStartProperty,
    InsetBlockStart,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetBlockEndProperty,
    InsetBlockEnd,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetInlineStartProperty,
    InsetInlineStart,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetInlineEndProperty,
    InsetInlineEnd,
    LengthPercentageAuto,
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    ColorProperty,
    Color,
    CssColor,
    "https://www.w3.org/TR/css-color-4/#propdef-color"
);
typed_property!(
    BackgroundColorProperty,
    BackgroundColor,
    CssColor,
    "https://www.w3.org/TR/css-backgrounds-3/#background-color"
);
typed_property!(
    BorderStartStartRadiusProperty,
    BorderStartStartRadius,
    BorderCornerRadius,
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    BorderStartEndRadiusProperty,
    BorderStartEndRadius,
    BorderCornerRadius,
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    BorderEndStartRadiusProperty,
    BorderEndStartRadius,
    BorderCornerRadius,
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    BorderEndEndRadiusProperty,
    BorderEndEndRadius,
    BorderCornerRadius,
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    FontWeightProperty,
    FontWeight,
    FontWeight,
    "https://www.w3.org/TR/css-fonts-4/#font-weight-prop"
);
typed_property!(
    OpacityProperty,
    Opacity,
    Opacity,
    "https://www.w3.org/TR/css-color-4/#transparency"
);
typed_property!(
    ZIndexProperty,
    ZIndex,
    ZIndex,
    "https://www.w3.org/TR/CSS22/visuren.html#z-index"
);
typed_property!(
    ForcedColorAdjustProperty,
    ForcedColorAdjust,
    ForcedColorAdjust,
    "https://www.w3.org/TR/css-color-adjust-1/#forced-color-adjust-prop"
);
typed_property!(
    PrintColorAdjustProperty,
    PrintColorAdjust,
    PrintColorAdjust,
    "https://www.w3.org/TR/css-color-adjust-1/#print-color-adjust"
);
typed_property!(
    ViewTransitionNameProperty,
    ViewTransitionName,
    ViewTransitionName,
    "https://www.w3.org/TR/css-view-transitions-1/#view-transition-name-prop"
);

/// ```compile_fail
/// use leptos_css::{LengthPercentageAuto, Padding, property::PaddingProperty};
///
/// // `auto` has no constructor path into `Padding`.
/// PaddingProperty.declare(Padding::All(LengthPercentageAuto::Auto));
/// ```
///
/// ```compile_fail
/// use leptos_css::{px, property::TouchActionProperty};
///
/// // A dimension is not in the `touch-action` grammar.
/// TouchActionProperty.declare(px(10));
/// ```
///
/// ```compile_fail
/// use leptos::tachys::html::style::style;
/// use leptos_css::{LengthPercentageAuto, PropertyName};
///
/// // The name catalog cannot be used as an unchecked Leptos property key.
/// let _ = style((PropertyName::Padding, LengthPercentageAuto::Auto));
/// ```
///
/// ```compile_fail
/// use leptos_css::CssValue;
///
/// // There is no arbitrary-string escape hatch in the typed value union.
/// let _ = CssValue::Str("auto".into());
/// ```
///
/// ```compile_fail
/// use leptos_css::{CssColor, CssColorName, CssCustomProperty, Size, property::WidthProperty, var};
///
/// let accent = CssCustomProperty::<CssColor>::new("--accent-color");
/// // A color variable remains a color and cannot enter the width grammar.
/// WidthProperty.declare(var(&accent, CssColor::Named(CssColorName::Black)));
/// ```
///
/// ```compile_fail
/// use leptos_css::{CssColor, CssCustomProperty, Size, var};
///
/// let accent = CssCustomProperty::<CssColor>::new("--accent-color");
/// // The fallback must have the variable's declared color grammar.
/// let _ = var(&accent, Size::Auto);
/// ```
///
/// ```compile_fail
/// use leptos_css::{CssColor, CssCustomProperty, var};
///
/// let accent = CssCustomProperty::<CssColor>::new("--accent-color");
/// // Checked variable references always require a fallback.
/// let _ = var(&accent);
/// ```
///
/// ```compile_fail
/// use leptos_css::{CssColor, CssColorName, var};
///
/// // Raw names cannot enter the typed `var()` constructor.
/// let _ = var("--accent-color", CssColor::Named(CssColorName::Black));
/// ```
///
/// ```compile_fail
/// use leptos_css::{CssColor, CssCustomProperty, Size};
///
/// let accent = CssCustomProperty::<CssColor>::new("--accent-color");
/// // A typed custom property only accepts its declared grammar.
/// let _ = accent.declare(Size::Auto);
/// ```
///
/// ```compile_fail
/// // Legacy, speech/aural, and SVG-paint properties are intentionally not checked APIs.
/// use leptos_css::property::{GridGapProperty, SpeakProperty, FloodColorProperty};
/// ```
const _: () = ();
