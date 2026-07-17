#![doc = include_str!("../README.md")]

mod angle;
mod box_model;
mod checked_value;
mod color;
mod color_adjust;
mod custom_property;
mod declaration;
mod dimension;
mod gap;
mod global_keyword;
mod identifier;
mod length;
mod number;
mod opacity;
/// Canonical property names and grammar-checked declaration selectors.
pub mod property;
mod serialization;
mod sizing;
mod stacking;
mod time;
mod touch_action;
mod typography;
mod value;
mod view_transition;

pub use angle::CssAngle;
pub use box_model::{
    BorderCornerRadius, Inset, InsetAxis, InvalidNonNegativeLengthPercentage, Margin, MarginAxis,
    NonNegativeLengthPercentage, NonNegativeLengthPercentageValue, Padding, PaddingAxis,
};
pub use checked_value::CheckedCssValue;
pub use color::{CssColor, CssColorName, hsl, hsla, rgb, rgba, try_hsl, try_hsla, try_rgba};
pub use color_adjust::{ForcedColorAdjust, PrintColorAdjust};
pub use custom_property::{
    CssCustomProperty, CssVariableReference, DeclarationValue, InvalidCustomPropertyName, var,
};
pub use declaration::CheckedDeclaration;
pub use dimension::{
    CssDimension, CssDimensionExpr, CssEnvironmentVariable, LengthPercentageAuto,
    LengthPercentageCalculation, ch, cqh, cqw, css_clamp, css_env, css_max, css_min, dvh, dvw, em,
    lvh, lvw, pct, px, rem, svh, svw, try_ch, try_cqh, try_cqw, try_dvh, try_dvw, try_em, try_lvh,
    try_lvw, try_pct, try_px, try_rem, try_svh, try_svw, try_vh, try_vmax, try_vmin, try_vw, vh,
    vmax, vmin, vw,
};
pub use gap::{Gap, GapValue};
pub use global_keyword::GlobalKeyword;
pub use length::CssLength;
pub use number::{
    FiniteF64, InvalidCssNumber, NonNegativeFiniteF64, PercentageChannel, UnitInterval,
};
pub use opacity::Opacity;
pub use property::PropertyName;
pub use serialization::CssWriteTo;
pub use sizing::{MaxSize, Size};
pub use stacking::ZIndex;
pub use time::CssTime;
pub use touch_action::{
    TouchAction, TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan,
};
pub use typography::FontWeight;
pub use value::{
    CssValue, deg, fr, grad, ms, number, rad, s, try_deg, try_fr, try_grad, try_ms, try_number,
    try_rad, try_s, try_turn, turn,
};
pub use view_transition::{InvalidViewTransitionName, ViewTransitionName};

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
