use std::borrow::Cow;

use crate::{
    BorderCornerRadius, CheckedCssValue, CheckedDeclaration, CssColor, CssCustomProperty,
    DeclarationValue, FontWeight, ForcedColorAdjust, Gap, GapValue, GlobalKeyword, Inset,
    InsetAxis, LengthPercentageAuto, Margin, MarginAxis, MaxSize, NonNegativeLengthPercentageValue,
    Opacity, Padding, PaddingAxis, PrintColorAdjust, Size, TouchAction, ViewTransitionName, ZIndex,
};

/// A non-exhaustive catalog of CSS property names.
///
/// Each variant maps to its kebab-case CSS property name via [`PropertyName::as_str`].
/// `PropertyName` does not
/// implement `AsRef<str>` and therefore cannot be paired with an arbitrary value in a Leptos
/// style tuple. Use this module's property-specific selectors to construct checked declarations.
///
/// Only canonical, modern, general-purpose property names belong in this catalog. Deprecated
/// properties, legacy aliases, and SVG-only rendering or paint hints are intentionally omitted,
/// even when a compatibility specification still indexes them.
///
/// Variants are intentionally undocumented, as they should be self-explanatory.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PropertyName {
    AlignContent,
    AlignItems,
    AlignSelf,
    All,
    Animation,
    AnimationDelay,
    AnimationDirection,
    AnimationDuration,
    AnimationFillMode,
    AnimationIterationCount,
    AnimationName,
    AnimationPlayState,
    AnimationTimingFunction,
    Appearance,
    AspectRatio,
    BackdropFilter,
    BackfaceVisibility,
    Background,
    BackgroundAttachment,
    BackgroundBlendMode,
    BackgroundClip,
    BackgroundColor,
    BackgroundImage,
    BackgroundOrigin,
    BackgroundPosition,
    BackgroundRepeat,
    BackgroundSize,
    BlockSize,
    Border,
    BorderBlock,
    BorderBlockColor,
    BorderBlockEnd,
    BorderBlockEndColor,
    BorderBlockEndStyle,
    BorderBlockEndWidth,
    BorderBlockStart,
    BorderBlockStartColor,
    BorderBlockStartStyle,
    BorderBlockStartWidth,
    BorderBlockStyle,
    BorderBlockWidth,
    BorderBottom,
    BorderBottomColor,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderBottomStyle,
    BorderBottomWidth,
    BorderCollapse,
    BorderColor,
    BorderEndEndRadius,
    BorderEndStartRadius,
    BorderImage,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageSlice,
    BorderImageSource,
    BorderImageWidth,
    BorderInline,
    BorderInlineColor,
    BorderInlineEnd,
    BorderInlineEndColor,
    BorderInlineEndStyle,
    BorderInlineEndWidth,
    BorderInlineStart,
    BorderInlineStartColor,
    BorderInlineStartStyle,
    BorderInlineStartWidth,
    BorderInlineStyle,
    BorderInlineWidth,
    BorderLeft,
    BorderLeftColor,
    BorderLeftStyle,
    BorderLeftWidth,
    BorderRadius,
    BorderRight,
    BorderRightColor,
    BorderRightStyle,
    BorderRightWidth,
    BorderSpacing,
    BorderStartEndRadius,
    BorderStartStartRadius,
    BorderStyle,
    BorderTop,
    BorderTopColor,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderTopStyle,
    BorderTopWidth,
    BorderWidth,
    Bottom,
    BoxDecorationBreak,
    BoxShadow,
    BoxSizing,
    BreakAfter,
    BreakBefore,
    BreakInside,
    CaptionSide,
    CaretColor,
    Clear,
    ClipPath,
    Color,
    ColorScheme,
    ColumnCount,
    ColumnFill,
    ColumnGap,
    ColumnRule,
    ColumnRuleColor,
    ColumnRuleStyle,
    ColumnRuleWidth,
    ColumnSpan,
    ColumnWidth,
    Columns,
    Contain,
    Content,
    ContentVisibility,
    CounterIncrement,
    CounterReset,
    CounterSet,
    Cursor,
    Direction,
    Display,
    EmptyCells,
    Filter,
    Flex,
    FlexBasis,
    FlexDirection,
    FlexFlow,
    FlexGrow,
    FlexShrink,
    FlexWrap,
    Float,
    ForcedColorAdjust,
    Font,
    FontFamily,
    FontFeatureSettings,
    FontKerning,
    FontOpticalSizing,
    FontSize,
    FontSizeAdjust,
    FontStyle,
    FontSynthesis,
    FontVariant,
    FontVariantCaps,
    FontVariantEastAsian,
    FontVariantLigatures,
    FontVariantNumeric,
    FontVariationSettings,
    FontWeight,
    FontWidth,
    Gap,
    Grid,
    GridArea,
    GridAutoColumns,
    GridAutoFlow,
    GridAutoRows,
    GridColumn,
    GridColumnEnd,
    GridColumnStart,
    GridRow,
    GridRowEnd,
    GridRowStart,
    GridTemplate,
    GridTemplateAreas,
    GridTemplateColumns,
    GridTemplateRows,
    Height,
    Hyphens,
    ImageOrientation,
    ImageRendering,
    InlineSize,
    Inset,
    InsetBlock,
    InsetBlockEnd,
    InsetBlockStart,
    InsetInline,
    InsetInlineEnd,
    InsetInlineStart,
    Isolation,
    JustifyContent,
    JustifyItems,
    JustifySelf,
    Left,
    LetterSpacing,
    LineBreak,
    LineHeight,
    ListStyle,
    ListStyleImage,
    ListStylePosition,
    ListStyleType,
    Margin,
    MarginBlock,
    MarginBlockEnd,
    MarginBlockStart,
    MarginBottom,
    MarginInline,
    MarginInlineEnd,
    MarginInlineStart,
    MarginLeft,
    MarginRight,
    MarginTop,
    Mask,
    MaskImage,
    MaxBlockSize,
    MaxHeight,
    MaxInlineSize,
    MaxWidth,
    MinBlockSize,
    MinHeight,
    MinInlineSize,
    MinWidth,
    MixBlendMode,
    ObjectFit,
    ObjectPosition,
    Offset,
    OffsetAnchor,
    OffsetDistance,
    OffsetPath,
    OffsetRotate,
    Opacity,
    Order,
    Orphans,
    Outline,
    OutlineColor,
    OutlineOffset,
    OutlineStyle,
    OutlineWidth,
    Overflow,
    OverflowAnchor,
    OverflowWrap,
    OverflowX,
    OverflowY,
    OverscrollBehavior,
    OverscrollBehaviorBlock,
    OverscrollBehaviorInline,
    OverscrollBehaviorX,
    OverscrollBehaviorY,
    Padding,
    PaddingBlock,
    PaddingBlockEnd,
    PaddingBlockStart,
    PaddingBottom,
    PaddingInline,
    PaddingInlineEnd,
    PaddingInlineStart,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
    Perspective,
    PerspectiveOrigin,
    PlaceContent,
    PlaceItems,
    PlaceSelf,
    PointerEvents,
    Position,
    PrintColorAdjust,
    Quotes,
    Resize,
    Right,
    Rotate,
    RowGap,
    Scale,
    ScrollBehavior,
    ScrollMargin,
    ScrollMarginBlock,
    ScrollMarginBlockEnd,
    ScrollMarginBlockStart,
    ScrollMarginBottom,
    ScrollMarginInline,
    ScrollMarginInlineEnd,
    ScrollMarginInlineStart,
    ScrollMarginLeft,
    ScrollMarginRight,
    ScrollMarginTop,
    ScrollPadding,
    ScrollPaddingBlock,
    ScrollPaddingBlockEnd,
    ScrollPaddingBlockStart,
    ScrollPaddingBottom,
    ScrollPaddingInline,
    ScrollPaddingInlineEnd,
    ScrollPaddingInlineStart,
    ScrollPaddingLeft,
    ScrollPaddingRight,
    ScrollPaddingTop,
    ScrollSnapAlign,
    ScrollSnapStop,
    ScrollSnapType,
    ScrollbarColor,
    ScrollbarGutter,
    ScrollbarWidth,
    ShapeImageThreshold,
    ShapeMargin,
    ShapeOutside,
    TabSize,
    TableLayout,
    TextAlign,
    TextAlignLast,
    TextCombineUpright,
    TextDecoration,
    TextDecorationColor,
    TextDecorationLine,
    TextDecorationSkipInk,
    TextDecorationStyle,
    TextDecorationThickness,
    TextEmphasis,
    TextEmphasisColor,
    TextEmphasisPosition,
    TextEmphasisStyle,
    TextIndent,
    TextJustify,
    TextOrientation,
    TextOverflow,
    TextShadow,
    TextTransform,
    TextUnderlineOffset,
    TextUnderlinePosition,
    Top,
    TouchAction,
    Transform,
    TransformBox,
    TransformOrigin,
    TransformStyle,
    Transition,
    TransitionDelay,
    TransitionDuration,
    TransitionProperty,
    TransitionTimingFunction,
    Translate,
    UnicodeBidi,
    UserSelect,
    VerticalAlign,
    ViewTransitionName,
    Visibility,
    WhiteSpace,
    Widows,
    Width,
    WillChange,
    WordBreak,
    WordSpacing,
    WritingMode,
    ZIndex,
}

impl PropertyName {
    /// Returns the CSS property name as a static string slice.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AlignContent => "align-content",
            Self::AlignItems => "align-items",
            Self::AlignSelf => "align-self",
            Self::All => "all",
            Self::Animation => "animation",
            Self::AnimationDelay => "animation-delay",
            Self::AnimationDirection => "animation-direction",
            Self::AnimationDuration => "animation-duration",
            Self::AnimationFillMode => "animation-fill-mode",
            Self::AnimationIterationCount => "animation-iteration-count",
            Self::AnimationName => "animation-name",
            Self::AnimationPlayState => "animation-play-state",
            Self::AnimationTimingFunction => "animation-timing-function",
            Self::Appearance => "appearance",
            Self::AspectRatio => "aspect-ratio",
            Self::BackdropFilter => "backdrop-filter",
            Self::BackfaceVisibility => "backface-visibility",
            Self::Background => "background",
            Self::BackgroundAttachment => "background-attachment",
            Self::BackgroundBlendMode => "background-blend-mode",
            Self::BackgroundClip => "background-clip",
            Self::BackgroundColor => "background-color",
            Self::BackgroundImage => "background-image",
            Self::BackgroundOrigin => "background-origin",
            Self::BackgroundPosition => "background-position",
            Self::BackgroundRepeat => "background-repeat",
            Self::BackgroundSize => "background-size",
            Self::BlockSize => "block-size",
            Self::Border => "border",
            Self::BorderBlock => "border-block",
            Self::BorderBlockColor => "border-block-color",
            Self::BorderBlockEnd => "border-block-end",
            Self::BorderBlockEndColor => "border-block-end-color",
            Self::BorderBlockEndStyle => "border-block-end-style",
            Self::BorderBlockEndWidth => "border-block-end-width",
            Self::BorderBlockStart => "border-block-start",
            Self::BorderBlockStartColor => "border-block-start-color",
            Self::BorderBlockStartStyle => "border-block-start-style",
            Self::BorderBlockStartWidth => "border-block-start-width",
            Self::BorderBlockStyle => "border-block-style",
            Self::BorderBlockWidth => "border-block-width",
            Self::BorderBottom => "border-bottom",
            Self::BorderBottomColor => "border-bottom-color",
            Self::BorderBottomLeftRadius => "border-bottom-left-radius",
            Self::BorderBottomRightRadius => "border-bottom-right-radius",
            Self::BorderBottomStyle => "border-bottom-style",
            Self::BorderBottomWidth => "border-bottom-width",
            Self::BorderCollapse => "border-collapse",
            Self::BorderColor => "border-color",
            Self::BorderEndEndRadius => "border-end-end-radius",
            Self::BorderEndStartRadius => "border-end-start-radius",
            Self::BorderImage => "border-image",
            Self::BorderImageOutset => "border-image-outset",
            Self::BorderImageRepeat => "border-image-repeat",
            Self::BorderImageSlice => "border-image-slice",
            Self::BorderImageSource => "border-image-source",
            Self::BorderImageWidth => "border-image-width",
            Self::BorderInline => "border-inline",
            Self::BorderInlineColor => "border-inline-color",
            Self::BorderInlineEnd => "border-inline-end",
            Self::BorderInlineEndColor => "border-inline-end-color",
            Self::BorderInlineEndStyle => "border-inline-end-style",
            Self::BorderInlineEndWidth => "border-inline-end-width",
            Self::BorderInlineStart => "border-inline-start",
            Self::BorderInlineStartColor => "border-inline-start-color",
            Self::BorderInlineStartStyle => "border-inline-start-style",
            Self::BorderInlineStartWidth => "border-inline-start-width",
            Self::BorderInlineStyle => "border-inline-style",
            Self::BorderInlineWidth => "border-inline-width",
            Self::BorderLeft => "border-left",
            Self::BorderLeftColor => "border-left-color",
            Self::BorderLeftStyle => "border-left-style",
            Self::BorderLeftWidth => "border-left-width",
            Self::BorderRadius => "border-radius",
            Self::BorderRight => "border-right",
            Self::BorderRightColor => "border-right-color",
            Self::BorderRightStyle => "border-right-style",
            Self::BorderRightWidth => "border-right-width",
            Self::BorderSpacing => "border-spacing",
            Self::BorderStartEndRadius => "border-start-end-radius",
            Self::BorderStartStartRadius => "border-start-start-radius",
            Self::BorderStyle => "border-style",
            Self::BorderTop => "border-top",
            Self::BorderTopColor => "border-top-color",
            Self::BorderTopLeftRadius => "border-top-left-radius",
            Self::BorderTopRightRadius => "border-top-right-radius",
            Self::BorderTopStyle => "border-top-style",
            Self::BorderTopWidth => "border-top-width",
            Self::BorderWidth => "border-width",
            Self::Bottom => "bottom",
            Self::BoxDecorationBreak => "box-decoration-break",
            Self::BoxShadow => "box-shadow",
            Self::BoxSizing => "box-sizing",
            Self::BreakAfter => "break-after",
            Self::BreakBefore => "break-before",
            Self::BreakInside => "break-inside",
            Self::CaptionSide => "caption-side",
            Self::CaretColor => "caret-color",
            Self::Clear => "clear",
            Self::ClipPath => "clip-path",
            Self::Color => "color",
            Self::ColorScheme => "color-scheme",
            Self::ColumnCount => "column-count",
            Self::ColumnFill => "column-fill",
            Self::ColumnGap => "column-gap",
            Self::ColumnRule => "column-rule",
            Self::ColumnRuleColor => "column-rule-color",
            Self::ColumnRuleStyle => "column-rule-style",
            Self::ColumnRuleWidth => "column-rule-width",
            Self::ColumnSpan => "column-span",
            Self::ColumnWidth => "column-width",
            Self::Columns => "columns",
            Self::Contain => "contain",
            Self::Content => "content",
            Self::ContentVisibility => "content-visibility",
            Self::CounterIncrement => "counter-increment",
            Self::CounterReset => "counter-reset",
            Self::CounterSet => "counter-set",
            Self::Cursor => "cursor",
            Self::Direction => "direction",
            Self::Display => "display",
            Self::EmptyCells => "empty-cells",
            Self::Filter => "filter",
            Self::Flex => "flex",
            Self::FlexBasis => "flex-basis",
            Self::FlexDirection => "flex-direction",
            Self::FlexFlow => "flex-flow",
            Self::FlexGrow => "flex-grow",
            Self::FlexShrink => "flex-shrink",
            Self::FlexWrap => "flex-wrap",
            Self::Float => "float",
            Self::ForcedColorAdjust => "forced-color-adjust",
            Self::Font => "font",
            Self::FontFamily => "font-family",
            Self::FontFeatureSettings => "font-feature-settings",
            Self::FontKerning => "font-kerning",
            Self::FontOpticalSizing => "font-optical-sizing",
            Self::FontSize => "font-size",
            Self::FontSizeAdjust => "font-size-adjust",
            Self::FontStyle => "font-style",
            Self::FontSynthesis => "font-synthesis",
            Self::FontVariant => "font-variant",
            Self::FontVariantCaps => "font-variant-caps",
            Self::FontVariantEastAsian => "font-variant-east-asian",
            Self::FontVariantLigatures => "font-variant-ligatures",
            Self::FontVariantNumeric => "font-variant-numeric",
            Self::FontVariationSettings => "font-variation-settings",
            Self::FontWeight => "font-weight",
            Self::FontWidth => "font-width",
            Self::Gap => "gap",
            Self::Grid => "grid",
            Self::GridArea => "grid-area",
            Self::GridAutoColumns => "grid-auto-columns",
            Self::GridAutoFlow => "grid-auto-flow",
            Self::GridAutoRows => "grid-auto-rows",
            Self::GridColumn => "grid-column",
            Self::GridColumnEnd => "grid-column-end",
            Self::GridColumnStart => "grid-column-start",
            Self::GridRow => "grid-row",
            Self::GridRowEnd => "grid-row-end",
            Self::GridRowStart => "grid-row-start",
            Self::GridTemplate => "grid-template",
            Self::GridTemplateAreas => "grid-template-areas",
            Self::GridTemplateColumns => "grid-template-columns",
            Self::GridTemplateRows => "grid-template-rows",
            Self::Height => "height",
            Self::Hyphens => "hyphens",
            Self::ImageOrientation => "image-orientation",
            Self::ImageRendering => "image-rendering",
            Self::InlineSize => "inline-size",
            Self::Inset => "inset",
            Self::InsetBlock => "inset-block",
            Self::InsetBlockEnd => "inset-block-end",
            Self::InsetBlockStart => "inset-block-start",
            Self::InsetInline => "inset-inline",
            Self::InsetInlineEnd => "inset-inline-end",
            Self::InsetInlineStart => "inset-inline-start",
            Self::Isolation => "isolation",
            Self::JustifyContent => "justify-content",
            Self::JustifyItems => "justify-items",
            Self::JustifySelf => "justify-self",
            Self::Left => "left",
            Self::LetterSpacing => "letter-spacing",
            Self::LineBreak => "line-break",
            Self::LineHeight => "line-height",
            Self::ListStyle => "list-style",
            Self::ListStyleImage => "list-style-image",
            Self::ListStylePosition => "list-style-position",
            Self::ListStyleType => "list-style-type",
            Self::Margin => "margin",
            Self::MarginBlock => "margin-block",
            Self::MarginBlockEnd => "margin-block-end",
            Self::MarginBlockStart => "margin-block-start",
            Self::MarginBottom => "margin-bottom",
            Self::MarginInline => "margin-inline",
            Self::MarginInlineEnd => "margin-inline-end",
            Self::MarginInlineStart => "margin-inline-start",
            Self::MarginLeft => "margin-left",
            Self::MarginRight => "margin-right",
            Self::MarginTop => "margin-top",
            Self::Mask => "mask",
            Self::MaskImage => "mask-image",
            Self::MaxBlockSize => "max-block-size",
            Self::MaxHeight => "max-height",
            Self::MaxInlineSize => "max-inline-size",
            Self::MaxWidth => "max-width",
            Self::MinBlockSize => "min-block-size",
            Self::MinHeight => "min-height",
            Self::MinInlineSize => "min-inline-size",
            Self::MinWidth => "min-width",
            Self::MixBlendMode => "mix-blend-mode",
            Self::ObjectFit => "object-fit",
            Self::ObjectPosition => "object-position",
            Self::Offset => "offset",
            Self::OffsetAnchor => "offset-anchor",
            Self::OffsetDistance => "offset-distance",
            Self::OffsetPath => "offset-path",
            Self::OffsetRotate => "offset-rotate",
            Self::Opacity => "opacity",
            Self::Order => "order",
            Self::Orphans => "orphans",
            Self::Outline => "outline",
            Self::OutlineColor => "outline-color",
            Self::OutlineOffset => "outline-offset",
            Self::OutlineStyle => "outline-style",
            Self::OutlineWidth => "outline-width",
            Self::Overflow => "overflow",
            Self::OverflowAnchor => "overflow-anchor",
            Self::OverflowWrap => "overflow-wrap",
            Self::OverflowX => "overflow-x",
            Self::OverflowY => "overflow-y",
            Self::OverscrollBehavior => "overscroll-behavior",
            Self::OverscrollBehaviorBlock => "overscroll-behavior-block",
            Self::OverscrollBehaviorInline => "overscroll-behavior-inline",
            Self::OverscrollBehaviorX => "overscroll-behavior-x",
            Self::OverscrollBehaviorY => "overscroll-behavior-y",
            Self::Padding => "padding",
            Self::PaddingBlock => "padding-block",
            Self::PaddingBlockEnd => "padding-block-end",
            Self::PaddingBlockStart => "padding-block-start",
            Self::PaddingBottom => "padding-bottom",
            Self::PaddingInline => "padding-inline",
            Self::PaddingInlineEnd => "padding-inline-end",
            Self::PaddingInlineStart => "padding-inline-start",
            Self::PaddingLeft => "padding-left",
            Self::PaddingRight => "padding-right",
            Self::PaddingTop => "padding-top",
            Self::Perspective => "perspective",
            Self::PerspectiveOrigin => "perspective-origin",
            Self::PlaceContent => "place-content",
            Self::PlaceItems => "place-items",
            Self::PlaceSelf => "place-self",
            Self::PointerEvents => "pointer-events",
            Self::Position => "position",
            Self::PrintColorAdjust => "print-color-adjust",
            Self::Quotes => "quotes",
            Self::Resize => "resize",
            Self::Right => "right",
            Self::Rotate => "rotate",
            Self::RowGap => "row-gap",
            Self::Scale => "scale",
            Self::ScrollBehavior => "scroll-behavior",
            Self::ScrollMargin => "scroll-margin",
            Self::ScrollMarginBlock => "scroll-margin-block",
            Self::ScrollMarginBlockEnd => "scroll-margin-block-end",
            Self::ScrollMarginBlockStart => "scroll-margin-block-start",
            Self::ScrollMarginBottom => "scroll-margin-bottom",
            Self::ScrollMarginInline => "scroll-margin-inline",
            Self::ScrollMarginInlineEnd => "scroll-margin-inline-end",
            Self::ScrollMarginInlineStart => "scroll-margin-inline-start",
            Self::ScrollMarginLeft => "scroll-margin-left",
            Self::ScrollMarginRight => "scroll-margin-right",
            Self::ScrollMarginTop => "scroll-margin-top",
            Self::ScrollPadding => "scroll-padding",
            Self::ScrollPaddingBlock => "scroll-padding-block",
            Self::ScrollPaddingBlockEnd => "scroll-padding-block-end",
            Self::ScrollPaddingBlockStart => "scroll-padding-block-start",
            Self::ScrollPaddingBottom => "scroll-padding-bottom",
            Self::ScrollPaddingInline => "scroll-padding-inline",
            Self::ScrollPaddingInlineEnd => "scroll-padding-inline-end",
            Self::ScrollPaddingInlineStart => "scroll-padding-inline-start",
            Self::ScrollPaddingLeft => "scroll-padding-left",
            Self::ScrollPaddingRight => "scroll-padding-right",
            Self::ScrollPaddingTop => "scroll-padding-top",
            Self::ScrollSnapAlign => "scroll-snap-align",
            Self::ScrollSnapStop => "scroll-snap-stop",
            Self::ScrollSnapType => "scroll-snap-type",
            Self::ScrollbarColor => "scrollbar-color",
            Self::ScrollbarGutter => "scrollbar-gutter",
            Self::ScrollbarWidth => "scrollbar-width",
            Self::ShapeImageThreshold => "shape-image-threshold",
            Self::ShapeMargin => "shape-margin",
            Self::ShapeOutside => "shape-outside",
            Self::TabSize => "tab-size",
            Self::TableLayout => "table-layout",
            Self::TextAlign => "text-align",
            Self::TextAlignLast => "text-align-last",
            Self::TextCombineUpright => "text-combine-upright",
            Self::TextDecoration => "text-decoration",
            Self::TextDecorationColor => "text-decoration-color",
            Self::TextDecorationLine => "text-decoration-line",
            Self::TextDecorationSkipInk => "text-decoration-skip-ink",
            Self::TextDecorationStyle => "text-decoration-style",
            Self::TextDecorationThickness => "text-decoration-thickness",
            Self::TextEmphasis => "text-emphasis",
            Self::TextEmphasisColor => "text-emphasis-color",
            Self::TextEmphasisPosition => "text-emphasis-position",
            Self::TextEmphasisStyle => "text-emphasis-style",
            Self::TextIndent => "text-indent",
            Self::TextJustify => "text-justify",
            Self::TextOrientation => "text-orientation",
            Self::TextOverflow => "text-overflow",
            Self::TextShadow => "text-shadow",
            Self::TextTransform => "text-transform",
            Self::TextUnderlineOffset => "text-underline-offset",
            Self::TextUnderlinePosition => "text-underline-position",
            Self::Top => "top",
            Self::TouchAction => "touch-action",
            Self::Transform => "transform",
            Self::TransformBox => "transform-box",
            Self::TransformOrigin => "transform-origin",
            Self::TransformStyle => "transform-style",
            Self::Transition => "transition",
            Self::TransitionDelay => "transition-delay",
            Self::TransitionDuration => "transition-duration",
            Self::TransitionProperty => "transition-property",
            Self::TransitionTimingFunction => "transition-timing-function",
            Self::Translate => "translate",
            Self::UnicodeBidi => "unicode-bidi",
            Self::UserSelect => "user-select",
            Self::VerticalAlign => "vertical-align",
            Self::ViewTransitionName => "view-transition-name",
            Self::Visibility => "visibility",
            Self::WhiteSpace => "white-space",
            Self::Widows => "widows",
            Self::Width => "width",
            Self::WillChange => "will-change",
            Self::WordBreak => "word-break",
            Self::WordSpacing => "word-spacing",
            Self::WritingMode => "writing-mode",
            Self::ZIndex => "z-index",
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A sealed selector for one CSS property and its exact Rust value grammar.
///
/// Implementations are provided by this crate for its checked property surface and for
/// [`CssCustomProperty`]. External code cannot introduce arbitrary property names into checked
/// declarations.
///
/// ```compile_fail
/// use leptos_css::{CheckedDeclaration, CssColor, DeclarationValue};
/// use leptos_css::property::CheckedProperty;
///
/// #[derive(Clone)]
/// struct ExternalProperty;
///
/// impl CheckedProperty for ExternalProperty {
///     type Value = CssColor;
///
///     fn property_name(&self) -> &str {
///         "external-property"
///     }
///
///     fn declare<V>(&self, _value: V) -> CheckedDeclaration
///     where
///         V: Into<DeclarationValue<Self::Value>>,
///     {
///         unreachable!()
///     }
/// }
/// ```
pub trait CheckedProperty: sealed::Sealed + Clone + Send + Sync + 'static {
    /// The value grammar accepted by this property.
    type Value: CheckedCssValue;

    /// Return the CSS property name selected by this value.
    fn property_name(&self) -> &str;

    /// Construct a checked declaration for this property.
    fn declare<V>(&self, value: V) -> CheckedDeclaration
    where
        V: Into<DeclarationValue<Self::Value>>;
}

/// A checked property that also accepts CSS-wide keywords.
pub trait CssWideProperty: CheckedProperty {
    /// Construct a checked declaration using a CSS-wide keyword.
    fn declare_global(&self, value: GlobalKeyword) -> CheckedDeclaration;
}

/// Generate a property selector whose `declare` method accepts one exact CSS grammar.
macro_rules! typed_property {
    ($name:ident, $variant:ident, $value:ty, $css_name:literal, $spec:literal) => {
        #[doc = concat!("Checked selector for the `", $css_name, "` property.")]
        #[doc = concat!("\n\nAccepts the [`", stringify!($value), "`] grammar.")]
        #[doc = concat!("\n\n[Normative specification](", $spec, ").")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $name {
            /// Construct a grammar-checked declaration for this property.
            pub fn declare(
                &self,
                value: impl Into<DeclarationValue<$value>>,
            ) -> CheckedDeclaration {
                <Self as CheckedProperty>::declare(self, value)
            }

            /// Construct a checked declaration using a CSS-wide keyword.
            pub fn declare_global(&self, value: GlobalKeyword) -> CheckedDeclaration {
                <Self as CssWideProperty>::declare_global(self, value)
            }
        }

        impl sealed::Sealed for $name {}

        impl CheckedProperty for $name {
            type Value = $value;

            fn property_name(&self) -> &str {
                PropertyName::$variant.as_str()
            }

            fn declare<V>(&self, value: V) -> CheckedDeclaration
            where
                V: Into<DeclarationValue<Self::Value>>,
            {
                CheckedDeclaration::new(
                    Cow::Borrowed(PropertyName::$variant.as_str()),
                    value.into(),
                )
            }
        }

        impl CssWideProperty for $name {
            fn declare_global(&self, value: GlobalKeyword) -> CheckedDeclaration {
                CheckedDeclaration::new(Cow::Borrowed(PropertyName::$variant.as_str()), value)
            }
        }
    };
}

impl<T> CssCustomProperty<T>
where
    T: CheckedCssValue + 'static,
{
    /// Create a typed declaration assigning this custom property a checked value.
    ///
    /// The value may be a literal of `T` or a typed [`crate::var`] reference whose fallback has
    /// the same grammar. Arbitrary custom-property token strings are intentionally unsupported.
    pub fn declare<V>(&self, value: V) -> CheckedDeclaration
    where
        V: Into<DeclarationValue<T>>,
    {
        <Self as CheckedProperty>::declare(self, value)
    }
}

impl<T> sealed::Sealed for CssCustomProperty<T> {}

impl<T> CheckedProperty for CssCustomProperty<T>
where
    T: CheckedCssValue + 'static,
{
    type Value = T;

    fn property_name(&self) -> &str {
        self.as_str()
    }

    fn declare<V>(&self, value: V) -> CheckedDeclaration
    where
        V: Into<DeclarationValue<Self::Value>>,
    {
        CheckedDeclaration::new(self.cloned_name(), value.into())
    }
}

/// Checked selector for the [`all` property](https://www.w3.org/TR/css-cascade-6/#all-shorthand),
/// whose grammar consists only of CSS-wide keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllProperty;

impl AllProperty {
    /// Construct a grammar-checked `all` declaration.
    pub fn declare(&self, value: GlobalKeyword) -> CheckedDeclaration {
        <Self as CheckedProperty>::declare(self, value)
    }
}

impl sealed::Sealed for AllProperty {}

impl CheckedProperty for AllProperty {
    type Value = GlobalKeyword;

    fn property_name(&self) -> &str {
        PropertyName::All.as_str()
    }

    fn declare<V>(&self, value: V) -> CheckedDeclaration
    where
        V: Into<DeclarationValue<Self::Value>>,
    {
        CheckedDeclaration::new(Cow::Borrowed(PropertyName::All.as_str()), value.into())
    }
}

typed_property!(
    TouchActionProperty,
    TouchAction,
    TouchAction,
    "touch-action",
    "https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property"
);
typed_property!(
    PaddingProperty,
    Padding,
    Padding,
    "padding",
    "https://www.w3.org/TR/css-box-4/#padding-shorthand"
);
typed_property!(
    PaddingTopProperty,
    PaddingTop,
    NonNegativeLengthPercentageValue,
    "padding-top",
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingRightProperty,
    PaddingRight,
    NonNegativeLengthPercentageValue,
    "padding-right",
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingBottomProperty,
    PaddingBottom,
    NonNegativeLengthPercentageValue,
    "padding-bottom",
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingLeftProperty,
    PaddingLeft,
    NonNegativeLengthPercentageValue,
    "padding-left",
    "https://www.w3.org/TR/css-box-4/#padding-physical"
);
typed_property!(
    PaddingBlockStartProperty,
    PaddingBlockStart,
    NonNegativeLengthPercentageValue,
    "padding-block-start",
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingBlockEndProperty,
    PaddingBlockEnd,
    NonNegativeLengthPercentageValue,
    "padding-block-end",
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingInlineStartProperty,
    PaddingInlineStart,
    NonNegativeLengthPercentageValue,
    "padding-inline-start",
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingInlineEndProperty,
    PaddingInlineEnd,
    NonNegativeLengthPercentageValue,
    "padding-inline-end",
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingBlockProperty,
    PaddingBlock,
    PaddingAxis,
    "padding-block",
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    PaddingInlineProperty,
    PaddingInline,
    PaddingAxis,
    "padding-inline",
    "https://www.w3.org/TR/css-logical-1/#padding-properties"
);
typed_property!(
    MarginProperty,
    Margin,
    Margin,
    "margin",
    "https://www.w3.org/TR/css-box-4/#margin-shorthand"
);
typed_property!(
    MarginTopProperty,
    MarginTop,
    LengthPercentageAuto,
    "margin-top",
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginRightProperty,
    MarginRight,
    LengthPercentageAuto,
    "margin-right",
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginBottomProperty,
    MarginBottom,
    LengthPercentageAuto,
    "margin-bottom",
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginLeftProperty,
    MarginLeft,
    LengthPercentageAuto,
    "margin-left",
    "https://www.w3.org/TR/css-box-4/#margin-physical"
);
typed_property!(
    MarginBlockStartProperty,
    MarginBlockStart,
    LengthPercentageAuto,
    "margin-block-start",
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginBlockEndProperty,
    MarginBlockEnd,
    LengthPercentageAuto,
    "margin-block-end",
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginInlineStartProperty,
    MarginInlineStart,
    LengthPercentageAuto,
    "margin-inline-start",
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginInlineEndProperty,
    MarginInlineEnd,
    LengthPercentageAuto,
    "margin-inline-end",
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginBlockProperty,
    MarginBlock,
    MarginAxis,
    "margin-block",
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    MarginInlineProperty,
    MarginInline,
    MarginAxis,
    "margin-inline",
    "https://www.w3.org/TR/css-logical-1/#margin-properties"
);
typed_property!(
    GapProperty,
    Gap,
    Gap,
    "gap",
    "https://www.w3.org/TR/css-align-3/#gap-shorthand"
);
typed_property!(
    RowGapProperty,
    RowGap,
    GapValue,
    "row-gap",
    "https://www.w3.org/TR/css-align-3/#column-row-gap"
);
typed_property!(
    ColumnGapProperty,
    ColumnGap,
    GapValue,
    "column-gap",
    "https://www.w3.org/TR/css-align-3/#column-row-gap"
);
typed_property!(
    WidthProperty,
    Width,
    Size,
    "width",
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    HeightProperty,
    Height,
    Size,
    "height",
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    BlockSizeProperty,
    BlockSize,
    Size,
    "block-size",
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    InlineSizeProperty,
    InlineSize,
    Size,
    "inline-size",
    "https://www.w3.org/TR/css-sizing-4/#preferred-size-properties"
);
typed_property!(
    MinWidthProperty,
    MinWidth,
    Size,
    "min-width",
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MinHeightProperty,
    MinHeight,
    Size,
    "min-height",
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MinBlockSizeProperty,
    MinBlockSize,
    Size,
    "min-block-size",
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MinInlineSizeProperty,
    MinInlineSize,
    Size,
    "min-inline-size",
    "https://www.w3.org/TR/css-sizing-4/#min-size-properties"
);
typed_property!(
    MaxWidthProperty,
    MaxWidth,
    MaxSize,
    "max-width",
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    MaxHeightProperty,
    MaxHeight,
    MaxSize,
    "max-height",
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    MaxBlockSizeProperty,
    MaxBlockSize,
    MaxSize,
    "max-block-size",
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    MaxInlineSizeProperty,
    MaxInlineSize,
    MaxSize,
    "max-inline-size",
    "https://www.w3.org/TR/css-sizing-4/#max-size-properties"
);
typed_property!(
    TopProperty,
    Top,
    LengthPercentageAuto,
    "top",
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    RightProperty,
    Right,
    LengthPercentageAuto,
    "right",
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    BottomProperty,
    Bottom,
    LengthPercentageAuto,
    "bottom",
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    LeftProperty,
    Left,
    LengthPercentageAuto,
    "left",
    "https://www.w3.org/TR/css-position-3/#inset-properties"
);
typed_property!(
    InsetProperty,
    Inset,
    Inset,
    "inset",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetBlockProperty,
    InsetBlock,
    InsetAxis,
    "inset-block",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetInlineProperty,
    InsetInline,
    InsetAxis,
    "inset-inline",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetBlockStartProperty,
    InsetBlockStart,
    LengthPercentageAuto,
    "inset-block-start",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetBlockEndProperty,
    InsetBlockEnd,
    LengthPercentageAuto,
    "inset-block-end",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetInlineStartProperty,
    InsetInlineStart,
    LengthPercentageAuto,
    "inset-inline-start",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    InsetInlineEndProperty,
    InsetInlineEnd,
    LengthPercentageAuto,
    "inset-inline-end",
    "https://www.w3.org/TR/css-logical-1/#inset-properties"
);
typed_property!(
    ColorProperty,
    Color,
    CssColor,
    "color",
    "https://www.w3.org/TR/css-color-4/#propdef-color"
);
typed_property!(
    BackgroundColorProperty,
    BackgroundColor,
    CssColor,
    "background-color",
    "https://www.w3.org/TR/css-backgrounds-3/#background-color"
);
typed_property!(
    BorderStartStartRadiusProperty,
    BorderStartStartRadius,
    BorderCornerRadius,
    "border-start-start-radius",
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    BorderStartEndRadiusProperty,
    BorderStartEndRadius,
    BorderCornerRadius,
    "border-start-end-radius",
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    BorderEndStartRadiusProperty,
    BorderEndStartRadius,
    BorderCornerRadius,
    "border-end-start-radius",
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    BorderEndEndRadiusProperty,
    BorderEndEndRadius,
    BorderCornerRadius,
    "border-end-end-radius",
    "https://www.w3.org/TR/css-backgrounds-3/#border-radius"
);
typed_property!(
    FontWeightProperty,
    FontWeight,
    FontWeight,
    "font-weight",
    "https://www.w3.org/TR/css-fonts-4/#font-weight-prop"
);
typed_property!(
    OpacityProperty,
    Opacity,
    Opacity,
    "opacity",
    "https://www.w3.org/TR/css-color-4/#transparency"
);
typed_property!(
    ZIndexProperty,
    ZIndex,
    ZIndex,
    "z-index",
    "https://www.w3.org/TR/CSS22/visuren.html#z-index"
);
typed_property!(
    ForcedColorAdjustProperty,
    ForcedColorAdjust,
    ForcedColorAdjust,
    "forced-color-adjust",
    "https://www.w3.org/TR/css-color-adjust-1/#forced-color-adjust-prop"
);
typed_property!(
    PrintColorAdjustProperty,
    PrintColorAdjust,
    PrintColorAdjust,
    "print-color-adjust",
    "https://www.w3.org/TR/css-color-adjust-1/#print-color-adjust"
);
typed_property!(
    ViewTransitionNameProperty,
    ViewTransitionName,
    ViewTransitionName,
    "view-transition-name",
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
