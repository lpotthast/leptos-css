# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0]

### Added

- Typed CSS primitives and closed declaration grammars for finite numbers, lengths, percentages, angles, times,
  colors, fractional units, font weights, box-model values, sizing, touch actions, color adjustment, view
  transitions, opacity, and stacking order. Public enums are non-exhaustive.
- Validated numeric wrappers and fallible `try_` constructors that reject non-finite, negative, or out-of-range
  values wherever the CSS grammar imposes those constraints.
- Convenience constructors for 16 length units, percentages, angles, times, fractional units, and RGB, RGBA, HSL,
  and HSLA colors. Color constructors return `CssColor` directly so they compose with every checked color property.
- A `CssDimension` type limited to `<length-percentage>`, plus distinct `LengthPercentageAuto` and non-negative
  grammar types for properties with different constraints.
- Typed length-percentage expression trees with arithmetic and helpers for CSS `min()`, `max()`, `clamp()`, and
  supported `env()` variables. Range-restricted calculations retain CSS math-function range checking after folding
  and substitution.
- Validated, grammar-typed custom-property declarations and `var()` references with mandatory, matching,
  recursively nestable fallbacks.
- Typed margin, padding, gap, inset, logical-axis, sizing, `fit-content()`, and logical corner-radius values,
  including calculated forms where valid.
- A non-exhaustive `PropertyName` catalog that maps a curated set of modern CSS property names, including `all`, to
  kebab-case text.
- A sealed family of property-specific selectors for checked declarations of `all`, padding, margin, gap, sizing,
  inset, color, background color, logical corner radii, font weight, opacity, touch action, z-index, color adjustment,
  and view-transition names, with CSS-wide keywords where valid and property-specific normative specification links.
- An erasable `CheckedDeclaration` that retains its validated property/value pairing and integrates directly with
  Leptos style attributes for server-side rendering, hydration, browser updates, and reset behavior.
- A fail-closed checked API without arbitrary property/value strings, unchecked CSS function bodies, untyped
  variable references, or grammar-independent style pairings.
- Shared `CssWriteTo` and `Display` serialization for rendering typed values and complete declarations to CSS text.
- A `nightly` feature forwarding to Leptos, a Rust 1.89.0 minimum supported version, and compile-time `Send + Sync`
  checks for core public types.
- Checked values reserve a small initial serialization buffer, and static custom-property declarations retain
  borrowed names without cloning them into owned strings.

[Unreleased]: https://github.com/lpotthast/leptos-css/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/lpotthast/leptos-css/releases/tag/v0.1.0
