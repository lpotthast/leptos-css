use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::serialization::CssWriteTo;

/// A horizontal panning component of the `touch-action` grammar.
///
/// [Pointer Events Level 4 `touch-action` grammar]: https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TouchActionHorizontalPan {
    /// `pan-x`.
    PanX,
    /// `pan-left`.
    PanLeft,
    /// `pan-right`.
    PanRight,
}

impl CssWriteTo for TouchActionHorizontalPan {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::PanX => "pan-x",
            Self::PanLeft => "pan-left",
            Self::PanRight => "pan-right",
        })
    }
}

/// A vertical panning component of the `touch-action` grammar.
///
/// [Pointer Events Level 4 `touch-action` grammar]: https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TouchActionVerticalPan {
    /// `pan-y`.
    PanY,
    /// `pan-up`.
    PanUp,
    /// `pan-down`.
    PanDown,
}

impl CssWriteTo for TouchActionVerticalPan {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::PanY => "pan-y",
            Self::PanUp => "pan-up",
            Self::PanDown => "pan-down",
        })
    }
}

/// A non-empty composition of panning and pinch-zoom gestures for `touch-action`.
///
/// Horizontal and vertical panning come from Pointer Events Level 4. The WHATWG Compatibility
/// Standard augments that grammar with the independently composable `pinch-zoom` keyword.
///
/// [Pointer Events Level 4 `touch-action` grammar]: https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property
/// [WHATWG Compatibility `touch-action` extension]: https://compat.spec.whatwg.org/#touch-action
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TouchActionGestures {
    horizontal: Option<TouchActionHorizontalPan>,
    vertical: Option<TouchActionVerticalPan>,
    pinch_zoom: bool,
}

impl TouchActionGestures {
    /// Start a gesture composition with horizontal panning.
    pub const fn horizontal(horizontal: TouchActionHorizontalPan) -> Self {
        Self {
            horizontal: Some(horizontal),
            vertical: None,
            pinch_zoom: false,
        }
    }

    /// Start a gesture composition with vertical panning.
    pub const fn vertical(vertical: TouchActionVerticalPan) -> Self {
        Self {
            horizontal: None,
            vertical: Some(vertical),
            pinch_zoom: false,
        }
    }

    /// Start a gesture composition with pinch zooming.
    pub const fn pinch_zoom() -> Self {
        Self {
            horizontal: None,
            vertical: None,
            pinch_zoom: true,
        }
    }

    /// Add or replace the horizontal panning component.
    #[must_use]
    pub const fn with_horizontal(mut self, horizontal: TouchActionHorizontalPan) -> Self {
        self.horizontal = Some(horizontal);
        self
    }

    /// Add or replace the vertical panning component.
    #[must_use]
    pub const fn with_vertical(mut self, vertical: TouchActionVerticalPan) -> Self {
        self.vertical = Some(vertical);
        self
    }

    /// Add pinch zooming to the gesture composition.
    #[must_use]
    pub const fn with_pinch_zoom(mut self) -> Self {
        self.pinch_zoom = true;
        self
    }
}

impl CssWriteTo for TouchActionGestures {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        let mut has_previous = false;

        if let Some(horizontal) = self.horizontal {
            horizontal.css_fmt(w)?;
            has_previous = true;
        }
        if let Some(vertical) = self.vertical {
            if has_previous {
                w.write_char(' ')?;
            }
            vertical.css_fmt(w)?;
            has_previous = true;
        }
        if self.pinch_zoom {
            if has_previous {
                w.write_char(' ')?;
            }
            w.write_str("pinch-zoom")?;
        }

        Ok(())
    }
}

/// A standards-valid `touch-action` value.
///
/// Pointer Events Level 4 defines the property keywords and the horizontal and vertical panning
/// components. The WHATWG Compatibility Standard adds `pinch-zoom` to the composable gesture
/// branch represented by [`TouchActionGestures`].
///
/// [Pointer Events Level 4 `touch-action` grammar]: https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property
/// [WHATWG Compatibility `touch-action` extension]: https://compat.spec.whatwg.org/#touch-action
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TouchAction {
    /// `auto`.
    Auto,
    /// `none`.
    None,
    /// `manipulation`.
    Manipulation,
    /// A non-empty combination of horizontal panning, vertical panning, and pinch zooming.
    Gestures(TouchActionGestures),
}

impl From<TouchActionGestures> for TouchAction {
    fn from(value: TouchActionGestures) -> Self {
        Self::Gestures(value)
    }
}

impl CssWriteTo for TouchAction {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::None => w.write_str("none"),
            Self::Manipulation => w.write_str("manipulation"),
            Self::Gestures(value) => value.css_fmt(w),
        }
    }
}

impl_checked_value!(
    TouchActionHorizontalPan,
    TouchActionVerticalPan,
    TouchActionGestures,
    TouchAction,
);
