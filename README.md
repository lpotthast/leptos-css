# leptos-css

[![crates.io](https://img.shields.io/crates/v/leptos-css.svg)](https://crates.io/crates/leptos-css)
[![docs.rs](https://docs.rs/leptos-css/badge.svg)](https://docs.rs/leptos-css)
[![CI](https://github.com/lpotthast/leptos-css/actions/workflows/ci.yml/badge.svg)](https://github.com/lpotthast/leptos-css/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rust-1.89%2B-blue.svg)](https://www.rust-lang.org/)

Typed CSS primitives and property-specific declaration grammars for
[Leptos](https://leptos.dev). The checked API is deliberately fail-closed: unsupported CSS cannot be smuggled in as
an arbitrary value string, and a value can become a declaration only through a selector for a property whose
grammar it satisfies.

`leptos-css` is the typed foundation of the [`leptos-styles`](https://crates.io/crates/leptos-styles) family. Its
checked boundary is the owned `CheckedDeclaration` returned by a property selector. Integrations can store and pass
that concrete type without separating its checked property/value pairing.

## Features

- Closed value types for lengths, percentages, colors, angles, times, global keywords, and selected property grammars
- Constructor helpers (`px`, `pct`, `em`, `rem`, `vh`, `vw`, `deg`, `rgb`, `hsl`, ...) with finite-input validation,
  plus fallible `try_*` counterparts for runtime-derived values. Helpers return the narrowest modeled grammar, so
  `rgb()` and `hsl()` feed every selector accepting `CssColor` directly.
- Typed arithmetic and typed `css_min()`, `css_max()`, `css_clamp()`, and `css_env()` constructors; function bodies are
  expression trees rather than strings
- Validated, grammar-typed custom properties and `var()` references with mandatory typed fallbacks
- Property selectors for checked Leptos declarations, including physical and logical padding, margin, inset, sizing and
  radius properties, colors, opacity, color adjustment, view transitions, font weight, `touch-action`, and `z-index`
- A non-exhaustive `PropertyName` catalog with kebab-case `as_str()` output; it is not a declaration API
- Zero-allocation rendering through the `CssWriteTo` trait (`write_to(&mut String)`)

## Installation

```bash
cargo add leptos-css
```

`leptos-css` is compatible with Leptos 0.8 and requires Rust 1.89 or newer.

## Example

```rust
use leptos_css::{
    CssColor, CssColorName, CssWriteTo, LengthPercentageCalculation, Padding, Size, TouchAction,
    css_custom_property, pct, px, rgb, try_px, var,
    property::{ColorProperty, PaddingProperty, TouchActionProperty, WidthProperty},
};

css_custom_property!(ACCENT_COLOR: CssColor = "--accent-color");

let mut buf = String::new();
px(16).write_to(&mut buf);
assert_eq!(buf, "16px");

let measured_width = try_px(24.5)?;
assert_eq!(measured_width.to_string(), "24.5px");

let padding = Padding::all(px(16));
assert_eq!(padding.to_string(), "16px");
let _padding_declaration = PaddingProperty.declare(padding);
let _rgb_declaration = ColorProperty.declare(rgb(12, 34, 56));

let width = Size::Calculation(LengthPercentageCalculation::new(pct(100) - px(20)));
assert_eq!(width.to_string(), "calc(100% - 20px)");
let _width_declaration = WidthProperty.declare(width);

let _touch_declaration = TouchActionProperty.declare(TouchAction::PanXPanYPinchZoom);

let _accent_declaration = ACCENT_COLOR.declare(CssColor::Named(CssColorName::Aqua));
let _color_declaration = ColorProperty.declare(var(
    &ACCENT_COLOR,
    CssColor::Named(CssColorName::CurrentColor),
));

# Ok::<(), leptos_css::InvalidCssNumber>(())
```

## Design notes

- Typed numeric variants store constrained wrappers with private fields. Non-finite numbers, out-of-range color
  channels and opacity, negative `fr` values, and negative non-negative-length values are unrepresentable.
- Ergonomic constructors (`px`, `pct`, `deg`, `s`, `hsl`, ...) panic on invalid input. Their `try_*` counterparts
  return `InvalidCssNumber` for runtime-derived values, and `TryFrom<f64>` is available for bare `CssValue` numbers.
- `CssDimension` is only a `<length-percentage>`. Margin and inset properties use the separate
  `LengthPercentageAuto` grammar, while padding and gap use `NonNegativeLengthPercentageValue`; consequently `auto`
  and directly negative values are available only where the specification permits them. Range-restricted
  calculations remain valid CSS math functions and are clamped after evaluation.
- `CssCustomProperty<T>` validates its name and fixes one Rust grammar `T`. `var()` requires a matching typed fallback,
  so an undefined variable cannot invalidate the consuming declaration. Reconstructing the same CSS name with a
  different `T`, or overriding it through untyped external CSS, is outside the checked boundary.
- CSS-wide keywords are added through each ordinary selector's `declare_global()` method. `AllProperty` accepts only
  a CSS-wide keyword.
- `CssValue` has no arbitrary-string variant, typed primitives do not implement Leptos `IntoStyleValue`,
  `PropertyName` does not implement `AsRef<str>`, and `CheckedDeclaration` has no public raw constructor. These
  boundaries prevent accidental arbitrary property/value pairing through this crate.
- All enums are `#[non_exhaustive]` so adding variants is non-breaking.

## Coverage contract

Property coverage is intentionally incremental. `PropertyName` is a convenience catalog, not a claim to contain every
property in the evolving CSS index. The checked declaration surface currently includes only the markers documented
above; an unsupported property or value grammar is not expressible through that surface.

Custom properties and `var()` are supported only through validated names, typed values, and mandatory typed fallbacks.
Fallback-free references and free-form function bodies remain omitted because they can become invalid at
computed-value time. Applications can still use ordinary handwritten CSS or Leptos string styles when they
intentionally need unsupported CSS, but those declarations are outside `leptos-css`'s checked boundary.

The baseline is the [CSS Snapshot 2026 property index](https://www.w3.org/TR/css-2026/#properties), plus explicitly
documented modern modules used by the public API, including CSS Values Level 4, CSS Sizing Level 4, CSS Environment
Variables Level 1, and CSS View Transitions Level 1. Legacy aliases, speech/aural properties, SVG-specific paint
properties, and other niche families are intentionally not targets. Browser support is separate from grammar
validity.

## License

Dual licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
