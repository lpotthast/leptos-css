use std::fmt;

use crate::checked_value::impl_checked_value;
use crate::serialization::CssWriteTo;

/// CSS font-weight values.
///
/// Represents the `font-weight` CSS property as a typed enum rather than a raw string.
/// Numeric weights range from 100 (thinnest) to 900 (boldest), with keyword aliases
/// for common values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FontWeight {
    /// Weight 100 — Thin / Hairline.
    W100,
    /// Weight 200 — Extra Light / Ultra Light.
    W200,
    /// Weight 300 — Light.
    W300,
    /// Weight 400 — Normal / Regular (same as `WNormal`).
    W400,
    /// Weight 500 — Medium.
    W500,
    /// Weight 600 — Semi Bold / Demi Bold.
    W600,
    /// Weight 700 — Bold (same as `WBold`).
    W700,
    /// Weight 800 — Extra Bold / Ultra Bold.
    W800,
    /// Weight 900 — Black / Heavy.
    W900,
    /// Keyword `lighter` — one relative step lighter than the inherited weight.
    WLighter,
    /// Keyword `normal` — equivalent to weight 400.
    WNormal,
    /// Keyword `bold` — equivalent to weight 700.
    WBold,
    /// Keyword `bolder` — one relative step bolder than the inherited weight.
    WBolder,
}

impl CssWriteTo for FontWeight {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::W100 => w.write_str("100"),
            Self::W200 => w.write_str("200"),
            Self::W300 => w.write_str("300"),
            Self::W400 => w.write_str("400"),
            Self::W500 => w.write_str("500"),
            Self::W600 => w.write_str("600"),
            Self::W700 => w.write_str("700"),
            Self::W800 => w.write_str("800"),
            Self::W900 => w.write_str("900"),
            Self::WLighter => w.write_str("lighter"),
            Self::WNormal => w.write_str("normal"),
            Self::WBold => w.write_str("bold"),
            Self::WBolder => w.write_str("bolder"),
        }
    }
}

impl_checked_value!(FontWeight);
