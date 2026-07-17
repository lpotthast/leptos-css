//! CSS property-name catalog and grammar-checked selectors.

mod name;
mod selector;

pub use name::PropertyName;
pub use selector::{
    AllProperty, BackgroundColorProperty, BlockSizeProperty, BorderEndEndRadiusProperty,
    BorderEndStartRadiusProperty, BorderStartEndRadiusProperty, BorderStartStartRadiusProperty,
    BottomProperty, ColorProperty, ColumnGapProperty, FontWeightProperty,
    ForcedColorAdjustProperty, GapProperty, HeightProperty, InlineSizeProperty,
    InsetBlockEndProperty, InsetBlockProperty, InsetBlockStartProperty, InsetInlineEndProperty,
    InsetInlineProperty, InsetInlineStartProperty, InsetProperty, LeftProperty,
    MarginBlockEndProperty, MarginBlockProperty, MarginBlockStartProperty, MarginBottomProperty,
    MarginInlineEndProperty, MarginInlineProperty, MarginInlineStartProperty, MarginLeftProperty,
    MarginProperty, MarginRightProperty, MarginTopProperty, MaxBlockSizeProperty,
    MaxHeightProperty, MaxInlineSizeProperty, MaxWidthProperty, MinBlockSizeProperty,
    MinHeightProperty, MinInlineSizeProperty, MinWidthProperty, OpacityProperty,
    PaddingBlockEndProperty, PaddingBlockProperty, PaddingBlockStartProperty,
    PaddingBottomProperty, PaddingInlineEndProperty, PaddingInlineProperty,
    PaddingInlineStartProperty, PaddingLeftProperty, PaddingProperty, PaddingRightProperty,
    PaddingTopProperty, PrintColorAdjustProperty, RightProperty, RowGapProperty, TopProperty,
    TouchActionProperty, ViewTransitionNameProperty, WidthProperty, ZIndexProperty,
};
